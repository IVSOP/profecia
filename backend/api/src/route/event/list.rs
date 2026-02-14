use axum::{Json, debug_handler, extract::State};
use serde::{Deserialize, Serialize};

use crate::{AppState, error::AppResult, state::event::EventDto};

#[derive(Serialize, Deserialize)]
pub struct ListResponse {
    pub events: Vec<EventDto>,
}

#[debug_handler]
pub async fn handle(State(app_state): State<AppState>) -> AppResult<Json<ListResponse>> {
    let events = app_state.get_all_events().await?;
    Ok(Json(ListResponse { events }))
}
