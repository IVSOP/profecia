use axum::{debug_handler, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState, error::AppResult, route::extractors::ValidatedJson, state::event::MarketOptionDto,
};

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BuyOrderRequest {
    pub market_id: Uuid,
    pub user_id: Uuid,
    pub shares: i64,
    #[validate(range(min = 1, max = 100))]
    pub price_per_share: i64,
    pub option: MarketOptionDto,
}

#[debug_handler]
pub async fn handle(
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<BuyOrderRequest>,
) -> AppResult<()> {
    state
        .create_buy_order(
            request.market_id,
            request.user_id,
            request.shares,
            request.price_per_share,
            request.option,
        )
        .await
}
