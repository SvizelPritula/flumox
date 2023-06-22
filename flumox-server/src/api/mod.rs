mod session;
mod game;

use http::StatusCode;
pub use session::{login, me};
pub use game::view;

pub async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
