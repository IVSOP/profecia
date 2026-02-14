use axum::{
    debug_handler,
    extract::{Path, State},
};
use sea_orm::TransactionTrait;
use uuid::Uuid;

use crate::{
    AppState,
    error::{AppError, AppResult},
    route::extractors::CurrentUser,
};

#[debug_handler]
pub async fn handle(
    Path(buy_order_id): Path<Uuid>,
    State(app_state): State<AppState>,
    CurrentUser(user): CurrentUser,
) -> AppResult<()> {
    let txn = app_state.database.begin().await?;

    let Some(buy_order) = AppState::get_buy_order(&txn, buy_order_id).await? else {
        return Err(AppError::BuyOrderNotFound);
    };

    if buy_order.user_id != user.id {
        return Err(AppError::Unauthorized(
            "Não podes vender ordens de compra que não são tuas".to_string(),
        ));
    }

    AppState::cancel_buy_order(&txn, buy_order_id).await?;

    txn.commit().await?;

    Ok(())
}
