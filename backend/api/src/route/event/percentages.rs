use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    AppState,
    error::AppResult,
    state::market::{AllPercentagesDto, EventPercentagesDto},
};

#[debug_handler]
pub async fn handle_all(State(app_state): State<AppState>) -> AppResult<Json<AllPercentagesDto>> {
    let percentages = app_state.get_all_percentages().await?;
    Ok(Json(percentages))
}

#[debug_handler]
pub async fn handle(
    Path(event_id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> AppResult<Json<EventPercentagesDto>> {
    let percentages = app_state.get_event_percentages(event_id).await?;
    Ok(Json(percentages))
}
