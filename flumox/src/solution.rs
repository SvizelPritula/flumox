use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Solution {
    Alphanumeric { solution: String },
    Number { solution: i32 },
}

fn normalize(s: &str) -> impl Iterator<Item = char> + '_ {
    s.nfkd()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_uppercase())
}

fn fuzzy_equal(a: &str, b: &str) -> bool {
    Iterator::eq(normalize(a), normalize(b))
}

fn numberic_equal(a: &str, b: i32) -> bool {
    let a: String = a
        .nfkc()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect();

    if let Ok(a) = i32::from_str(&a) {
        a == b
    } else {
        false
    }
}

impl Solution {
    pub fn check(&self, ans: &str) -> bool {
        match self {
            Solution::Alphanumeric { solution } => fuzzy_equal(ans, solution),
            Solution::Number { solution } => numberic_equal(ans, *solution),
        }
    }
}

impl ToString for Solution {
    fn to_string(&self) -> String {
        match self {
            Solution::Alphanumeric { solution } => solution.to_owned(),
            Solution::Number { solution } => solution.to_string(),
        }
    }
}
