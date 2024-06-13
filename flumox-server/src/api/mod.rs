mod game;
mod session;
mod socket;

use axum::http::StatusCode;
pub use game::{submit, view};
pub use session::{login, me};
pub use socket::sync_socket;

pub async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
