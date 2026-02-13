use axum::{Json, debug_handler, extract::State};

use crate::{
    error::AppResult,
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
        .register_user(&payload.username, &payload.password)
        .await?;

    let session_id = state.create_session(user.id).await?;
    let response = AuthResponse { session_id, user };

    Ok(Json(response))
}
