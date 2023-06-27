mod game;
mod session;

pub use game::{submit, view};
use http::StatusCode;
pub use session::{login, me};

pub async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
