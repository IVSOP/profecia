use axum::{Json, debug_handler, extract::State};
use serde::Serialize;

use crate::{
    AppState,
    error::AppResult,
    route::extractors::CurrentUser,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AirdropStatusResponse {
    pub last_airdrop: Option<String>,
    /// Seconds remaining until the next airdrop is available. 0 means ready.
    pub seconds_until_available: u64,
    pub available: bool,
}

#[debug_handler]
pub async fn status(
    CurrentUser(user): CurrentUser,
    State(state): State<AppState>,
) -> AppResult<Json<AirdropStatusResponse>> {
    let (last_airdrop, seconds_remaining) = state.get_airdrop_status(user.id).await?;

    Ok(Json(AirdropStatusResponse {
        last_airdrop: last_airdrop.map(|ts| ts.to_rfc3339()),
        seconds_until_available: seconds_remaining,
        available: seconds_remaining == 0,
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
