use axum::Router;

use crate::AppState;

mod event;
mod extractors;
mod user;

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/api/event", event::router())
        .nest("/api/user", user::router())
        .with_state(state)
}
