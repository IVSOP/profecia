use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

mod create;
mod info;
mod list;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list::handle))
        .route("/{id}", get(info::handle))
        .route("/", post(create::handle))
}
