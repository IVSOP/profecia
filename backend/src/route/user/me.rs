use axum::{Json, debug_handler, extract::State};
use serde::Serialize;

use crate::{
    error::AppResult,
    route::{AppState, extractors::CurrentUser},
    state::user::UserDto,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeResponse {
    pub user: UserDto,
}

#[debug_handler]
pub async fn handle(
    CurrentUser(user): CurrentUser,
    _state: State<AppState>,
) -> AppResult<Json<MeResponse>> {
    Ok(Json(MeResponse {
        user: UserDto {
            id: user.id,
            username: user.username,
        },
    }))
}
