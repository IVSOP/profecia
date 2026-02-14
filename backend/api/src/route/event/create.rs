use axum::{Json, debug_handler, extract::State};

use crate::{
    AppState,
    error::AppResult,
    route::extractors::AdminUser,
    state::event::{EventDto, EventRequest},
};

#[debug_handler]
pub async fn handle(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(request): Json<EventRequest>,
) -> AppResult<Json<EventDto>> {
    let event = state.create_event(request).await?;

    Ok(Json(event))
}
