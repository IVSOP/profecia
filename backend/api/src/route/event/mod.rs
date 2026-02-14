use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

mod buyorder;
mod chart;
mod create;
mod info;
mod list;
mod percentages;
mod position;
mod resolve;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list::handle))
        .route("/{id}", get(info::handle))
        .route("/", post(create::handle))
        .route("/resolve/{market_id}", post(resolve::handle))
        .route("/percentages", get(percentages::handle_all))
        .route("/percentages/{event_id}", get(percentages::handle))
        .route("/chart/{event_id}", get(chart::handle))
        .nest("/buyorder", buyorder::router())
        .nest("/position", position::router())
}
