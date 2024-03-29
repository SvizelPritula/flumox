use std::ops::Add;

use time::{Duration, OffsetDateTime};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Always,
    Since(OffsetDateTime),
    Never,
}

impl Value {
    #[inline]
    pub fn to_bool(&self, at: OffsetDateTime) -> bool {
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

impl From<OffsetDateTime> for Value {
    fn from(value: OffsetDateTime) -> Self {
        Value::Since(value)
    }
}

impl From<Option<OffsetDateTime>> for Value {
    fn from(value: Option<OffsetDateTime>) -> Self {
        match value {
            Some(value) => Value::Since(value),
            None => Value::Never,
        }
    }
}
