use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Toast {
    pub message: Message,
    #[serde(rename = "type")]
    pub class: ToastType,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToastType {
    Danger,
    Warning,
    Success,
}

impl Toast {
    pub fn new(message: Message, class: ToastType) -> Self {
        Self { message, class }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "value")]
pub enum Message {
    Custom(String),
    SolutionCorrect,
    SolutionIncorrect,
    HintTaken,
}
