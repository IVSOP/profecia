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
}

#[debug_handler]
pub async fn status(
    CurrentUser(user): CurrentUser,
    State(state): State<AppState>,
) -> AppResult<Json<AirdropStatusResponse>> {
    let last_airdrop = state.get_last_airdrop(user.id).await?;

    Ok(Json(AirdropStatusResponse {
        last_airdrop: last_airdrop.map(|ts| ts.to_rfc3339()),
    }))
}

#[debug_handler]
pub async fn request(
    CurrentUser(user): CurrentUser,
    State(state): State<AppState>,
) -> AppResult<()> {
    state.request_airdrop(user.id).await?;
    Ok(())
}
