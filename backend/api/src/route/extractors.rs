use axum::{
    extract::{FromRequest, FromRequestParts, Json, Request, rejection::JsonRejection},
    http::request::Parts,
};
use axum_extra::extract::CookieJar;
use serde::de::DeserializeOwned;
use uuid::Uuid;
use validator::Validate;

use crate::{AppState, error::AppError, state::user::UserDto};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

pub const AUTH_SESSION_COOKIE_NAME: &str = "sessionId";

#[derive(Debug)]
pub struct CurrentUser(pub UserDto);

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let session_cookie = jar
            .get(AUTH_SESSION_COOKIE_NAME)
            .ok_or(AppError::Unauthorized("missing session cookie".to_string()))?;

        let session_id = Uuid::try_parse(session_cookie.value())
            .map_err(|_| AppError::Unauthorized("invalid session cookie".to_string()))?;

        let user = state
            .get_user_by_session_id(session_id)
            .await?
            .ok_or(AppError::Unauthorized("session is not valid".to_string()))?;

        Ok(Self(user))
    }
}
