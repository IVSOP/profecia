use axum::Router;

use crate::AppState;

mod event;
mod extractors;
mod user;

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/event", event::router())
        .nest("/user", user::router())
        .with_state(state)
}
