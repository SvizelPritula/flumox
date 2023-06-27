use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Toast {
    pub text: String,
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
    pub fn new(text: String, class: ToastType) -> Self {
        Self { text, class }
    }
}
