use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::AppState;

mod add_market;
mod buyorder;
mod chart;
mod create;
mod info;
mod list;
mod percentages;
mod position;
mod resolve;
mod update_event;
mod update_market;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list::handle))
        .route("/{id}", get(info::handle))
        .route("/", post(create::handle))
        .route("/{id}", patch(update_event::handle))
        .route("/{id}/market", post(add_market::handle))
        .route("/market/{market_id}", patch(update_market::handle))
        .route("/resolve/{market_id}", post(resolve::handle))
        .route("/percentages", get(percentages::handle_all))
        .route("/percentages/{event_id}", get(percentages::handle))
        .route("/chart/{event_id}", get(chart::handle))
        .nest("/buyorder", buyorder::router())
        .nest("/position", position::router())
}
