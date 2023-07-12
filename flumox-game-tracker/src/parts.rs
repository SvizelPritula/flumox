use axum::http::StatusCode;
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
