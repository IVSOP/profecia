use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::AppState;

mod buy;
mod cancel;
mod list;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(buy::handle))
        .route("/{market_id}", get(list::handle))
        .route("/{order_id}", delete(cancel::handle))
}
