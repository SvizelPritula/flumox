use time::OffsetDateTime;
use time_expr::{eval, EvalError, Resolve, Value};

struct SampleResolver;

impl Resolve for SampleResolver {
    fn resolve(&mut self, path: &[&str]) -> Result<Value, EvalError> {
        match path {
            &["now"] => {
                let time = OffsetDateTime::now_utc();

                Ok(Value::Since(time))
            }
            _ => Err(EvalError::UnknownPath),
        }
    }
}

fn main() {
    let expr = "(2023-05-07 13:00 +0 | never) + 5 h";

    match eval(expr, &mut SampleResolver) {
        Ok(Value::Always) => println!("always"),
        Ok(Value::Never) => println!("never"),
        Ok(Value::Since(time)) => println!("since {}", time),
        Err(error) => println!("Error: {error}"),
    }
}
