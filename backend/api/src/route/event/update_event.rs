use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    AppState,
    error::AppResult,
    route::extractors::AdminUser,
    state::event::{EventDto, UpdateEventRequest},
};

#[debug_handler]
pub async fn handle(
    _admin: AdminUser,
    Path(event_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(request): Json<UpdateEventRequest>,
) -> AppResult<Json<EventDto>> {
    let event = state.update_event(event_id, request).await?;
    Ok(Json(event))
}
