use time::macros::datetime;

use crate::{eval, EvalError, Resolve, Value};

struct EmptyResolver;

impl Resolve for EmptyResolver {
    fn resolve(&mut self, path: &[&str]) -> Result<Value, EvalError> {
        Err(EvalError::UnknownPath {
            path: path.join(".").into_boxed_str(),
        })
    }
}

#[test]
fn literal_date() {
    let value = eval("1990-12-25 12:00 +1", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Since(datetime!(1990-12-25 12:00 +1)));
}

#[test]
fn literal_always() {
    let value = eval("always", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Always);
}

#[test]
fn literal_never() {
    let value = eval("never", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Never);
}

#[test]
fn or_never_always() {
    let value = eval("never | always", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Always);
}

#[test]
fn or_never_date() {
    let value = eval("never | 2000-01-01 00:00 +0", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Since(datetime!(2000-01-01 00:00 +0)));
}

#[test]
fn or_always_date() {
    let value = eval("always | 2000-01-01 00:00 +0", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Always);
}

#[test]
fn or_date_date() {
    let value = eval(
        "2000-01-01 00:00:00 +0 | 2000-01-01 00:00:01 +0",
        &mut EmptyResolver,
    )
    .unwrap();
    assert_eq!(value, Value::Since(datetime!(2000-01-01 00:00:00 +0)));
}

#[test]
fn and_never_always() {
    let value = eval("never & always", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Never);
}

#[test]
fn and_never_date() {
    let value = eval("never & 2000-01-01 00:00 +0", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Never);
}

#[test]
fn and_always_date() {
    let value = eval("always & 2000-01-01 00:00 +0", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Since(datetime!(2000-01-01 00:00 +0)));
}

#[test]
fn and_date_date() {
    let value = eval(
        "2000-01-01 00:00:00 +0 & 2000-01-01 00:00:01 +0",
        &mut EmptyResolver,
    )
    .unwrap();
    assert_eq!(value, Value::Since(datetime!(2000-01-01 00:00:01 +0)));
}

#[test]
fn and_timezones() {
    let value = eval(
        "2000-01-01 00:00 +0 & 2000-01-01 00:00 -1",
        &mut EmptyResolver,
    )
    .unwrap();
    assert_eq!(value, Value::Since(datetime!(2000-01-01 00:00 -1)));
}

#[test]
fn add_never() {
    let value = eval("never + 1 h", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Never);
}

#[test]
fn add_always() {
    let value = eval("always + 1 h", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Always);
}

#[test]
fn add_date() {
    let value = eval("2000-01-01 00:00 +0 + 1d 2h 3m 4s", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Since(datetime!(2000-01-02 02:03:04 +0)));
}

#[test]
fn parens_one() {
    let value = eval("never & (never | always)", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Never);
}

#[test]
fn parens_two() {
    let value = eval("(never & never) | always", &mut EmptyResolver).unwrap();
    assert_eq!(value, Value::Always);
}

struct NeverResolver;

impl Resolve for NeverResolver {
    fn resolve(&mut self, path: &[&str]) -> Result<Value, EvalError> {
        assert!(path.iter().all(|n| n.chars().all(|c| c.is_alphanumeric())));

        Ok(Value::Never)
    }
}

#[test]
fn unicode() {
    let value = eval("á ", &mut NeverResolver).unwrap();
    assert_eq!(value, Value::Never);
}

#[test]
fn unicode_continue() {
    let value = eval("áá | b", &mut NeverResolver).unwrap();
    assert_eq!(value, Value::Never);
}
