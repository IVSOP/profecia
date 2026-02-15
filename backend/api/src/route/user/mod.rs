use std::sync::LazyLock;

use axum::{
    Router,
    routing::{get, post},
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{AppState, state::user::UserDto};

mod airdrop;
mod balance;
mod login;
mod logout;
mod me;
mod positions;
mod register;

pub static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z0-9_.]+$").expect("valid username regex"));

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest {
    #[validate(regex(path = *USERNAME_REGEX, message = "O nome de utilizador deve ter apenas letras, números, pontos e underscores"))]
    #[validate(length(
        min = 3,
        max = 32,
        message = "O nome de utilizador deve ter entre 3 e 32 caracteres"
    ))]
    pub username: String,
    #[validate(length(
        min = 5,
        max = 128,
        message = "A password deve ter entre 5 e 128 caracteres"
    ))]
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub session_id: Uuid,
    pub user: UserDto,
}

pub fn router() -> axum::Router<AppState> {
    Router::new()
        .route("/register", post(register::handle))
        .route("/login", post(login::handle))
        .route("/me", get(me::handle))
        .route("/logout", post(logout::handle))
        .route("/airdrop", get(airdrop::status))
        .route("/airdrop", post(airdrop::request))
        .route("/balance", get(balance::handle))
        .route("/positions", get(positions::handle))
}
