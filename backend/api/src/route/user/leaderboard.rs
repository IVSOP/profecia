use axum::{Json, debug_handler, extract::State};

use crate::{AppState, error::AppResult, state::leaderboard::LeaderboardResponse};

#[debug_handler]
pub async fn handle(
    State(app_state): State<AppState>,
) -> AppResult<Json<LeaderboardResponse>> {
    let leaderboard = app_state.get_leaderboard().await?;
    Ok(Json(leaderboard))
}
