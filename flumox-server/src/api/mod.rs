mod game;
mod session;
mod socket;

pub use game::{submit, view};
use http::StatusCode;
pub use session::{login, me};
pub use socket::sync_socket;

pub async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
