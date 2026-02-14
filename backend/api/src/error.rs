use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
    #[error(transparent)]
    DatabaseError(#[from] sea_orm::error::DbErr),
    #[error("User {0} already exists")]
    UserAlreadyExists(String),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Market not found")]
    MarketNotFound,
    #[error("User not found")]
    UserNotFound,
    #[error("Market already resolved")]
    MarketAlreadyResolved,
    #[error("Buy order not found")]
    BuyOrderNotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorBody {
            error: String,
        }

        let internal_server_error = (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Internal server error"),
        );

        let (status, error) = match &self {
            AppError::ValidationError(err) => (
                StatusCode::BAD_REQUEST,
                itertools::Itertools::intersperse(
                    err.field_errors()
                        .into_values()
                        .flatten()
                        .filter_map(|v| v.message.clone())
                        .map(|a| a.to_string()),
                    ", ".to_string(),
                )
                .collect(),
            ),
            AppError::AxumJsonRejection(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            AppError::DatabaseError(err) => {
                tracing::error!("Database error: {}", err);
                internal_server_error
            }
            AppError::UserAlreadyExists(username) => (
                StatusCode::BAD_REQUEST,
                format!("Utilizador {} já existe", username),
            ),
            AppError::AnyhowError(error) => {
                tracing::error!("Error: {}", error);
                internal_server_error
            }
            AppError::Unauthorized(error) => (StatusCode::UNAUTHORIZED, error.to_string()),
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Credenciais inválidas".to_string(),
            ),
            AppError::MarketNotFound => {
                (StatusCode::NOT_FOUND, "Mercado não encontrado".to_string())
            }
            AppError::UserNotFound => (
                StatusCode::NOT_FOUND,
                "Utilizador não encontrado".to_string(),
            ),
            AppError::MarketAlreadyResolved => {
                (StatusCode::BAD_REQUEST, "Mercado já resolvido".to_string())
            }
            AppError::BuyOrderNotFound => {
                (StatusCode::NOT_FOUND, "Compra não encontrada".to_string())
            }
        };

        let body = ErrorBody { error };

        (status, Json(body)).into_response()
    }
}
