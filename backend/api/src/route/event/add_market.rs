use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    AppState,
    error::AppResult,
    route::extractors::AdminUser,
    state::event::{MarketDto, MarketRequest},
};

#[debug_handler]
pub async fn handle(
    _admin: AdminUser,
    Path(event_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(request): Json<MarketRequest>,
) -> AppResult<Json<MarketDto>> {
    let market = state.add_market_to_event(event_id, request).await?;
    Ok(Json(market))
}
