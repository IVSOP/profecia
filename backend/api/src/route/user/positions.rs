use axum::{Json, debug_handler, extract::State};

use crate::{
    AppState, error::AppResult, route::extractors::CurrentUser, state::position::PositionDto,
};

#[debug_handler]
pub async fn handle(
    CurrentUser(user): CurrentUser,
    State(app_state): State<AppState>,
) -> AppResult<Json<Vec<PositionDto>>> {
    let positions = app_state.list_user_positions(user.id).await?;

    Ok(Json(positions))
}
