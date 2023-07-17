use std::{
    fmt::{Display, Formatter},
    fs::{self, File},
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use postgres_protocol::escape::escape_literal;
use serde::Deserialize;
use serde_json::Value;
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
    priority: i64,
    config: Value,
}

#[derive(Debug, Clone, Deserialize)]
struct Team {
    name: String,
    access_code: String,
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

#[derive(Debug, Clone, Parser)]
/// Prepares a seed from JSON5
struct Options {
    /// Input path (default stdin)
    input: Option<PathBuf>,
    /// Output path (default stdout)
    #[arg(long, short)]
    output: Option<PathBuf>,
}

impl Widget {
    pub fn seed(&self, w: &mut impl Write, game: Uuid) -> Result<()> {
        let id = Uuid::new_v4();

        let config = serde_json::to_string(&self.config)?;

        writeln!(
            w,
            "INSERT INTO widget (game, id, ident, priority, config) VALUES ({}, {}, {}, {}, {});",
            Escape(game),
            Escape(id),
            Escape(&self.ident),
            Escape(self.priority),
            Escape(config)
        )?;

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
    pub fn seed(&self, w: &mut impl Write) -> Result<()> {
        let id = Uuid::new_v4();

        writeln!(
            w,
            "INSERT INTO game (id, name) VALUES ({}, {});",
            Escape(id),
            Escape(&self.name)
        )?;

        for widget in &self.widgets {
            widget.seed(w, id)?;
        }

        for team in &self.teams {
            team.seed(w, id)?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let opts = Options::parse();

    let input = match opts.input {
        Some(path) => fs::read_to_string(path)?,
        None => {
            let mut string = String::new();
            stdin().lock().read_to_string(&mut string)?;
            string
        }
    };

    let game: Game = json5::from_str(&input)?;

    match opts.output {
        Some(path) => {
            let mut output = File::create(path)?;
            game.seed(&mut output)?;
        }
        None => {
            let mut output = stdout().lock();
            game.seed(&mut output)?;
        }
    }

    Ok(())
}
