use axum::{
    debug_handler,
    extract::{Path, State},
};
use sea_orm::{EntityTrait, TransactionTrait};
use solana_sdk::{signature::Keypair, signer::Signer};
use uuid::Uuid;

use serde::Serialize;

use crate::{
    AppState, entity,
    error::{AppError, AppResult},
    route::extractors::CurrentUser,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    pub transaction_urls: Vec<String>,
}

#[debug_handler]
pub async fn handle(
    Path(buy_order_id): Path<Uuid>,
    State(app_state): State<AppState>,
    CurrentUser(user): CurrentUser,
) -> AppResult<axum::Json<TransactionResponse>> {
    let txn = app_state.database.begin().await?;

    let Some(buy_order) = AppState::get_buy_order(&txn, buy_order_id).await? else {
        return Err(AppError::BuyOrderNotFound);
    };

    if buy_order.user_id != user.id {
        return Err(AppError::Unauthorized(
            "Não podes vender ordens de compra que não são tuas".to_string(),
        ));
    }

    let market = entity::market::Entity::find_by_id(buy_order.market_id)
        .one(&txn)
        .await?
        .ok_or(AppError::MarketNotFound)?;

    let user_model = entity::user::Entity::find_by_id(user.id)
        .one(&txn)
        .await?
        .ok_or(AppError::UserNotFound)?;

    let user_wallet = Keypair::from_base58_string(&user_model.wallet);

    let sig = AppState::cancel_buy_order(
        &txn,
        buy_order.id,
        buy_order.shares,
        buy_order.price_per_share,
        market.event_id,
        &user_wallet.pubkey(),
        &app_state.solana,
    )
    .await?;

    let tx_url = app_state.solana.get_transaction_url(&sig);

    txn.commit().await?;

    Ok(axum::Json(TransactionResponse {
        transaction_urls: vec![tx_url],
    }))
}
