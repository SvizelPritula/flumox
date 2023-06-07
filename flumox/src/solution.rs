use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Solution {
    Alphanumeric { answer: String },
    Number { answer: i32 },
}
