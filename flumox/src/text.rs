use serde::{Deserialize, Serialize};
use time_expr::EvalError;

use crate::{expr::Expr, ViewContext};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Text(Vec<Paragraph>);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConditionalParagraph {
    text: String,
    show: Expr,
    hide: Expr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
struct SimpleParagraph(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Paragraph {
    Simple(SimpleParagraph),
    Conditional(ConditionalParagraph),
}

impl Text {
    pub fn render(&self, ctx: &mut ViewContext) -> Result<Vec<String>, EvalError> {
        let mut result = Vec::new();

        for p in &self.0 {
            if let Some(p) = p.content(ctx)? {
                result.push(p.to_string());
            }
        }

        Ok(result)
    }
}

impl Paragraph {
    pub fn content(&self, ctx: &mut ViewContext) -> Result<Option<&str>, EvalError> {
        match self {
            Paragraph::Simple(p) => Ok(Some(&p.0)),
            Paragraph::Conditional(p) => {
                let show = ctx.eval(&p.show)?;
                let hide = ctx.eval(&p.hide)?;

                if show && !hide {
                    Ok(Some(&p.text))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
