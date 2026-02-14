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

#[debug_handler]
pub async fn handle(
    _admin: AdminUser,
    Path(market_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(request): Json<ResolveMarketRequest>,
) -> AppResult<()> {
    state.resolve_market(market_id, request.option).await
}
