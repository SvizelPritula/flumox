mod session;
mod view;

use http::StatusCode;
pub use session::{login, me};
pub use view::view;

pub async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
