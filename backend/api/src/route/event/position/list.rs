use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    AppState, error::AppResult, route::extractors::CurrentUser, state::position::PositionDto,
};

#[debug_handler]
pub async fn handle(
    Path(event_id): Path<Uuid>,
    CurrentUser(user): CurrentUser,
    State(app_state): State<AppState>,
) -> AppResult<Json<Vec<PositionDto>>> {
    let positions = app_state
        .list_user_positions_in_event(user.id, event_id)
        .await?;

    Ok(Json(positions))
}
