use axum::{Json, debug_handler, extract::State};
use serde::Serialize;

use crate::{AppState, error::AppResult, route::extractors::CurrentUser};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AirdropStatusResponse {
    /// ISO 8601 timestamp when the next airdrop becomes available, or null if available now.
    pub next_airdrop_at: Option<String>,
    pub available: bool,
}

#[debug_handler]
pub async fn status(
    CurrentUser(user): CurrentUser,
    State(state): State<AppState>,
) -> AppResult<Json<AirdropStatusResponse>> {
    let next_airdrop_at = state.get_airdrop_status(user.id).await?;

    Ok(Json(AirdropStatusResponse {
        available: next_airdrop_at.is_none(),
        next_airdrop_at: next_airdrop_at.map(|ts| ts.to_rfc3339()),
    }))
}

#[debug_handler]
pub async fn request(
    CurrentUser(user): CurrentUser,
    State(state): State<AppState>,
) -> AppResult<()> {
    // airdrop 10 USDC
    state.request_airdrop(user.id, 10 * 100).await?;
    Ok(())
}
