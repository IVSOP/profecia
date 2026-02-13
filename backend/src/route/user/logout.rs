use axum::{debug_handler, extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::{AppState, error::AppResult, route::extractors::AUTH_SESSION_COOKIE_NAME};

#[debug_handler]
pub async fn handle(jar: CookieJar, State(state): State<AppState>) -> AppResult<StatusCode> {
    if let Some(session_cookie) = jar.get(&AUTH_SESSION_COOKIE_NAME) {
        if let Ok(session_id) = Uuid::parse_str(session_cookie.value()) {
            state.delete_session(session_id).await?;
        }
    }

    Ok(StatusCode::NO_CONTENT)
}
