use axum::{Json, debug_handler, extract::State};

use crate::{
    error::{AppError, AppResult},
    route::{
        AppState,
        extractors::ValidatedJson,
        user::{AuthRequest, AuthResponse},
    },
};

#[debug_handler]
pub async fn handle(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<AuthRequest>,
) -> AppResult<Json<AuthResponse>> {
    let user = state
        .get_user_by_username(&payload.username)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    if !state.check_user_password(user.id, payload.password).await? {
        return Err(AppError::InvalidCredentials);
    }

    let session_id = state.create_session(user.id).await?;

    Ok(Json(AuthResponse { session_id, user }))
}
