use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, error::AppResult, state::event::EventDto};

#[derive(Serialize, Deserialize)]
pub struct InfoResponse {
    pub event: Option<EventDto>,
}

#[debug_handler]
pub async fn handle(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> AppResult<Json<InfoResponse>> {
    let event = app_state.get_event_by_id(id).await?;
    Ok(Json(InfoResponse { event }))
}
