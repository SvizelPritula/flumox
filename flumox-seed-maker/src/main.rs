use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    fs::{self, File},
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
};

use anyhow::{anyhow, bail, Result};
use clap::Parser;
use postgres_protocol::escape::escape_literal;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
struct Escape<T>(T);

impl<T> Display for Escape<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        escape_literal(&self.0.to_string()).fmt(f)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Widget {
    ident: String,
    config: Value,
}

#[derive(Debug, Clone, Deserialize)]
struct Team {
    name: String,
    access_code: String,
    #[serde(default = "empty_object")]
    attributes: Value,
}

#[derive(Debug, Clone, Deserialize)]
struct Game {
    name: String,
    #[serde(default)]
    widgets: Vec<Widget>,
    #[serde(default)]
    teams: Vec<Team>,
}

fn empty_object() -> Value {
    Value::Object(Map::new())
}

#[derive(Debug, Clone, Parser)]
/// Prepares a seed from JSON5
struct Options {
    /// Input path (default stdin)
    input: Option<PathBuf>,
    /// Output path (default stdout)
    #[arg(long, short)]
    output: Option<PathBuf>,

    /// Generate update statements
    #[arg(long, short)]
    patch: bool,

    /// Create / update game with this UUID
    #[arg(long, short)]
    game_id: Option<Uuid>,

    /// Act on widget with a given ident
    #[arg(long = "widget", short)]
    widgets: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum InvalidateMessage {
    Game { game: Uuid },
}

impl Widget {
    pub fn seed(&self, w: &mut impl Write, game: Uuid, index: usize) -> Result<()> {
        let id = Uuid::new_v4();

        let config = serde_json::to_string(&self.config)?;
        let priority = i64::try_from(index * 100)?;

        writeln!(
            w,
            "INSERT INTO widget (game, id, ident, priority, config) VALUES ({}, {}, {}, {}, {});",
            Escape(game),
            Escape(id),
            Escape(&self.ident),
            Escape(priority),
            Escape(config)
        )?;

        Ok(())
    }

    pub fn patch(&self, w: &mut impl Write, game: Uuid, index: Option<usize>) -> Result<()> {
        let config = serde_json::to_string(&self.config)?;

        if let Some(index) = index {
            let priority = i64::try_from(index * 100)?;

            writeln!(
                w,
                "UPDATE widget SET config = {}, priority = {} WHERE game = {} AND ident = {};",
                Escape(config),
                Escape(priority),
                Escape(game),
                Escape(&self.ident),
            )?;
        } else {
            writeln!(
                w,
                "UPDATE widget SET config = {} WHERE game = {} AND ident = {};",
                Escape(config),
                Escape(game),
                Escape(&self.ident),
            )?;
        }

        Ok(())
    }
}

impl Team {
    pub fn seed(&self, w: &mut impl Write, game: Uuid) -> Result<()> {
        let id = Uuid::new_v4();

        let attributes = serde_json::to_string(&self.attributes)?;

        writeln!(
            w,
            "INSERT INTO team (game, id, name, access_code, attributes) VALUES ({}, {}, {}, {}, {});",
            Escape(game),
            Escape(id),
            Escape(&self.name),
            Escape(&self.access_code),
            Escape(attributes)
        )?;

        Ok(())
    }
}

impl Game {
    pub fn seed(&self, w: &mut impl Write, id: Option<Uuid>) -> Result<()> {
        let id = id.unwrap_or_else(Uuid::new_v4);

        writeln!(w, "BEGIN;")?;
        writeln!(
            w,
            "INSERT INTO game (id, name) VALUES ({}, {});",
            Escape(id),
            Escape(&self.name)
        )?;

        for (i, widget) in self.widgets.iter().enumerate() {
            widget.seed(w, id, i)?;
        }

        for team in &self.teams {
            team.seed(w, id)?;
        }

        writeln!(w, "COMMIT;")?;

        Ok(())
    }

    pub fn patch(&self, w: &mut impl Write, id: Uuid, widgets: HashSet<String>) -> Result<()> {
        writeln!(w, "BEGIN;")?;

        for (i, widget) in self.widgets.iter().enumerate() {
            if widgets.is_empty() || widgets.contains(&widget.ident) {
                widget.patch(w, id, widgets.is_empty().then_some(i))?;
            }
        }

        let message = serde_json::to_string(&InvalidateMessage::Game { game: id })?;

        writeln!(w, "NOTIFY invalidate, {};", Escape(message))?;
        writeln!(w, "COMMIT;")?;

        Ok(())
    }
}

fn preprocess(game: &mut Game) -> Result<()> {
    fn replace_templates_in_string(
        str: &str,
        mut replacer: impl FnMut(String) -> Result<String>,
    ) -> Result<String> {
        let mut chars = str.chars().peekable();
        let mut result = String::new();

        while let Some(char) = chars.next() {
            if char == '@' && chars.next_if_eq(&'[').is_some() {
                let mut name = String::new();

                loop {
                    match chars.next() {
                        Some(']') => break,
                        Some(c) => name.push(c),
                        None => bail!("no ']' to close open '@['"),
                    }
                }

                result.push_str(&replacer(name)?);
            } else {
                result.push(char);
            }
        }

        Ok(result)
    }

    fn replace(value: &mut Value, idents: &[String], idx: usize) -> Result<()> {
        match value {
            Value::String(string) => {
                *string = replace_templates_in_string(&string, |s| {
                    let offset: isize = s.parse()?;
                    idx.checked_add_signed(offset)
                        .and_then(|i| idents.get(i))
                        .cloned()
                        .ok_or_else(|| anyhow!("invalid index {offset}"))
                })?;
            }
            Value::Array(values) => {
                for value in values {
                    replace(value, idents, idx)?;
                }
            }
            Value::Object(map) => {
                for value in map.values_mut() {
                    replace(value, idents, idx)?;
                }
            }
            Value::Null | Value::Bool(_) | Value::Number(_) => {}
        }

        Ok(())
    }

    let idents: Vec<String> = game.widgets.iter().map(|w| w.ident.clone()).collect();

    for (idx, widget) in game.widgets.iter_mut().enumerate() {
        replace(&mut widget.config, &idents, idx)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let opts = Options::parse();

    let input = match &opts.input {
        Some(path) => fs::read_to_string(path)?,
        None => {
            let mut string = String::new();
            stdin().lock().read_to_string(&mut string)?;
            string
        }
    };

    let mut game: Game = json5::from_str(&input)?;
    preprocess(&mut game)?;

    fn generate(mut output: impl Write, game: Game, opts: Options) -> Result<()> {
        if opts.patch {
            let Some(id) = opts.game_id else {
                bail!("Cannot generate patch without game id");
            };

            game.patch(&mut output, id, opts.widgets.into_iter().collect())
        } else {
            game.seed(&mut output, opts.game_id)
        }
    }

    match &opts.output {
        Some(path) => {
            let output = File::create(path)?;
            generate(output, game, opts)
        }
        None => {
            let output = stdout().lock();
            generate(output, game, opts)
        }
    }
}
