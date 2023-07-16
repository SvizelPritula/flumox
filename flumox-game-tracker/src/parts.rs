use axum::http::StatusCode;
use flumox::Action;
use maud::{html, Markup, DOCTYPE};

pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                title { (title) }
            }
            body { (body) }
        }
    }
}

pub fn not_found(what: &str) -> (StatusCode, Markup) {
    (
        StatusCode::NOT_FOUND,
        page(
            &format!("{what} not found"),
            html!(
                h1 {
                    (what)
                    " not found."
                }
            ),
        ),
    )
}

pub fn action_description(payload: &Action) -> Markup {
    match payload {
        Action::Answer(answer) => html!("Submitted answer " i { (answer.answer) }),
        Action::Hint(hint) => html!("Taken hint " b { (hint.ident) }),
        #[allow(unreachable_patterns)]
        _ => html!("Unknown action"),
    }
}
