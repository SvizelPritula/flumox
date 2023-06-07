use anyhow::Result;
use flumox::{dummy, Cache, Context, GameState};
use indexmap::IndexMap;
use time::macros::datetime;
use time_expr::Value;

#[tokio::main]
async fn main() -> Result<()> {
    let mut instances = IndexMap::new();

    instances.insert(
        String::from("a"),
        dummy("2023-01-01 12:00 +2", Some(datetime!(2023-01-01 12:26 +2))),
    );
    instances.insert(
        String::from("b"),
        dummy("(a.visible + 5 m) | a.solved", None),
    );
    instances.insert(
        String::from("c"),
        dummy("(a.solved & b.solved) | 2023-01-01 20:00 +2", None),
    );

    let game = GameState { instances };
    let mut cache = Cache::default();
    let mut context = Context::new(&game, &mut cache);

    let value = context.eval(&"c.visible".into())?;

    match value {
        Value::Always => println!("always"),
        Value::Since(t) => println!("since {t}"),
        Value::Never => println!("never"),
    }

    Ok(())
}
