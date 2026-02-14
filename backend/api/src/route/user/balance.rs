use axum::{Json, debug_handler, extract::State};
use serde::Serialize;

use crate::{AppState, error::AppResult, route::extractors::CurrentUser};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResponse {
    pub balance_cents: u64,
}

#[debug_handler]
pub async fn handle(
    CurrentUser(current_user): CurrentUser,
    State(state): State<AppState>,
) -> AppResult<Json<BalanceResponse>> {
    let balance_cents = match state.get_balance_in_cents(current_user.id).await {
        Ok(balance) => balance,
        Err(e) => {
            if e.to_string().contains("AccountNotFound") {
                0
            } else {
                return Err(e);
            }
        }
    };
    Ok(Json(BalanceResponse { balance_cents }))
}
