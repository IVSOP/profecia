use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    AppState,
    error::AppResult,
    route::extractors::AdminUser,
    state::event::{MarketDto, UpdateMarketRequest},
};

#[debug_handler]
pub async fn handle(
    _admin: AdminUser,
    Path(market_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(request): Json<UpdateMarketRequest>,
) -> AppResult<Json<MarketDto>> {
    let market = state.update_market(market_id, request).await?;
    Ok(Json(market))
}
