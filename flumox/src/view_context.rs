use std::cmp::min;

use time::OffsetDateTime;
use time_expr::{Value, EvalError};

use crate::{Environment, expr::Expr};

#[derive(Debug)]
pub struct ViewContext<'a> {
    pub time: &'a mut TimeTracker,
    pub env: Environment<'a>,
}

impl<'a> ViewContext<'a> {
    pub fn new(env: Environment<'a>, tracker: &'a mut TimeTracker) -> ViewContext<'a> {
        ViewContext { time: tracker, env }
    }

    pub fn eval(&mut self, expr: &Expr) -> Result<bool, EvalError> {
        let value = self.env.eval(expr)?;
        Ok(self.time.after(value))
    }
}

#[derive(Debug)]
pub struct TimeTracker {
    current: OffsetDateTime,
    next_change: Option<OffsetDateTime>,
}

impl TimeTracker {
    #[inline(always)]
    pub fn new(time: OffsetDateTime) -> Self {
        TimeTracker {
            current: time,
            next_change: None,
        }
    }

    #[inline]
    pub fn after(&mut self, time: Value) -> bool {
        match time {
            Value::Always => true,
            Value::Never => false,
            Value::Since(time) => {
                if self.current >= time {
                    true
                } else {
                    self.record_change(time);
                    false
                }
            }
        }
    }

    fn record_change(&mut self, time: OffsetDateTime) {
        if let Some(change) = &mut self.next_change {
            *change = min(*change, time)
        } else {
            self.next_change = Some(time)
        }
    }

    #[inline(always)]
    pub fn valid_until(&self) -> Option<OffsetDateTime> {
        self.next_change
    }
}
