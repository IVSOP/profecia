use axum::{Router, routing::get};

use crate::AppState;

mod list;

pub fn router() -> Router<AppState> {
    Router::new().route("/{event_id}", get(list::handle))
}
