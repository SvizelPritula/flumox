use axum::http::StatusCode;
use flumox::Action;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use time::OffsetDateTime;

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

pub fn datetime(time: OffsetDateTime) -> Markup {
    let timestamp = time.unix_timestamp_nanos() / 1000_000;

    html!(
        span.time data-time={(&timestamp)} {
            (time)
        }
    )
}

pub fn time_script() -> Markup {
    const CODE: &str = concat!(
        "for (let e of document.getElementsByClassName('time')) {",
            "e.innerText = new Date(parseInt(e.dataset.time)).toLocaleString([], { timeZoneName: 'short' })",
        "}"
    );

    html!(
        script {(PreEscaped(CODE))}
    )
}
