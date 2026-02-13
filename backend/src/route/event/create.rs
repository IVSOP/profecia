use axum::{Json, debug_handler, extract::State};

use crate::{
    AppState,
    error::AppResult,
    state::event::{EventDto, EventRequest},
};

#[debug_handler]
pub async fn handle(
    State(state): State<AppState>,
    Json(request): Json<EventRequest>,
) -> AppResult<Json<EventDto>> {
    let event = state.create_event(request).await?;

    Ok(Json(event))
}
