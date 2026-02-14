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

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list::handle))
        .route("/{id}", get(info::handle))
        .route("/", post(create::handle))
        .nest("/buyorder", buyorder::router())
        .nest("/position", position::router())
}
