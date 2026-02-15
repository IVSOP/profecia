use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState, error::AppResult, route::extractors::AdminUser, state::event::MarketOptionDto,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveMarketRequest {
    pub option: MarketOptionDto,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    pub transaction_urls: Vec<String>,
}

#[debug_handler]
pub async fn handle(
    _admin: AdminUser,
    Path(market_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(request): Json<ResolveMarketRequest>,
) -> AppResult<Json<TransactionResponse>> {
    let tx_urls = state.resolve_market(market_id, request.option).await?;

    Ok(Json(TransactionResponse {
        transaction_urls: tx_urls,
    }))
}
