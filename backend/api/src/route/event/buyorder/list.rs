use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{AppState, error::AppResult, state::buyorder::BuyOrderDto};

#[debug_handler]
pub async fn handle(
    Path(market_id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> AppResult<Json<Vec<BuyOrderDto>>> {
    let buy_orders = app_state.list_buy_orders(market_id).await?;
    Ok(Json(buy_orders))
}
