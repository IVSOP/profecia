use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

mod buyorder;
mod create;
mod info;
mod list;
mod position;
mod resolve;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list::handle))
        .route("/{id}", get(info::handle))
        .route("/", post(create::handle))
        .route("/resolve/{market_id}", post(resolve::handle))
        .nest("/buyorder", buyorder::router())
        .nest("/position", position::router())
}
