use std::ops::Add;

use time::{Duration, PrimitiveDateTime};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Always,
    Since(PrimitiveDateTime),
    Never,
}

impl Value {
    pub fn to_bool(&self, at: PrimitiveDateTime) -> bool {
        match *self {
            Value::Always => true,
            Value::Since(time) => at >= time,
            Value::Never => false,
        }
    }
}

impl Add<Duration> for Value {
    type Output = Value;

    fn add(self, rhs: Duration) -> Self::Output {
        match self {
            Value::Since(time) => match time.checked_add(rhs) {
                Some(time) => Value::Since(time),
                None => Value::Never,
            },
            other => other,
        }
    }
}
