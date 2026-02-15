use axum::{Json, debug_handler, extract::State};
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
    #[validate(range(min = 1, max = 10000, message = "O número máximo de ações é de 10000"))]
    pub shares: i64,
    #[validate(range(
        min = 1,
        max = 99,
        message = "Preço por ação deve estar compreendido entre 1 e 99."
    ))]
    pub price_per_share: i64,
    pub option: MarketOptionDto,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    pub transaction_urls: Vec<String>,
}

#[debug_handler]
pub async fn handle(
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<BuyOrderRequest>,
) -> AppResult<Json<TransactionResponse>> {
    let tx_urls = state
        .create_buy_order(
            request.market_id,
            request.user_id,
            request.shares,
            request.price_per_share,
            request.option,
        )
        .await?;

    Ok(Json(TransactionResponse {
        transaction_urls: tx_urls,
    }))
}
