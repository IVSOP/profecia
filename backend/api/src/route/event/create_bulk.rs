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
    Json(requests): Json<Vec<EventRequest>>,
) -> AppResult<Json<Vec<EventDto>>> {
    let mut events = Vec::with_capacity(requests.len());

    for request in requests {
        let event = state.create_event(request).await?;
        events.push(event);
    }

    Ok(Json(events))
}
