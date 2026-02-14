use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

mod buy;
mod cancel;
mod list;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(buy::handle))
        .route("/{market_id}", get(list::handle))
        .route("/cancel/{order_id}", post(cancel::handle))
}
