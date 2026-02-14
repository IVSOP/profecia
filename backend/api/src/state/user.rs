use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use solana_sdk::{signature::Keypair, signer::Signer};
use uuid::Uuid;

use crate::{
    AppState, entity,
    error::{AppError, AppResult},
    utils::password::{hash_password, verify_password},
};

/// Airdrop cooldown in minutes
const AIRDROP_COOLDOWN: chrono::Duration = chrono::Duration::seconds(10);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdentityDto {
    pub id: Uuid,
    pub hashed_password: String,
}

impl AppState {
    pub async fn get_user_by_id(&self, id: Uuid) -> AppResult<Option<UserDto>> {
        let Some(user) = entity::user::Entity::find_by_id(id)
            .one(&self.database)
            .await?
        else {
            return Ok(None);
        };

        let user_dto = UserDto {
            id: user.id,
            username: user.username,
        };

        Ok(Some(user_dto))
    }

    pub async fn get_user_by_username(&self, username: &str) -> AppResult<Option<UserDto>> {
        let Some(user) = entity::user::Entity::find_by_username(username)
            .one(&self.database)
            .await?
        else {
            return Ok(None);
        };

        let user_dto = UserDto {
            id: user.id,
            username: user.username,
        };

        Ok(Some(user_dto))
    }

    pub async fn register_user(&self, username: &str, raw_password: &str) -> AppResult<UserDto> {
        if let Some(_) = self.get_user_by_username(username).await? {
            return Err(AppError::UserAlreadyExists(username.to_string()));
        }

        let user_id = Uuid::new_v4();
        let hashed_password = hash_password(raw_password.to_string()).await?;

        let wallet = self.solana.init_new_wallet().await?;

        let user = entity::user::ActiveModel {
            id: Set(user_id),
            username: Set(username.to_string()),
            last_airdrop: Set(None),
            wallet: Set(wallet.to_base58_string()),
        }
        .insert(&self.database)
        .await?;

        entity::identity::ActiveModel {
            user_id: Set(user.id),
            password_hash: Set(hashed_password),
            ..Default::default()
        }
        .insert(&self.database)
        .await?;

        Ok(UserDto {
            id: user_id,
            username: username.to_string(),
        })
    }

    pub async fn get_user_identity_by_id(
        &self,
        user_id: Uuid,
    ) -> AppResult<Option<UserIdentityDto>> {
        let Some(identity) = entity::identity::Entity::find_by_id(user_id)
            .one(&self.database)
            .await?
        else {
            return Ok(None);
        };

        Ok(Some(UserIdentityDto {
            id: identity.user_id,
            hashed_password: identity.password_hash,
        }))
    }

    pub async fn check_user_password(
        &self,
        user_id: Uuid,
        raw_password: String,
    ) -> AppResult<bool> {
        let Some(identity) = self.get_user_identity_by_id(user_id).await? else {
            return Ok(false);
        };

        let valid = verify_password(raw_password, identity.hashed_password).await?;

        Ok(valid)
    }

    pub async fn get_airdrop_status(
        &self,
        user_id: Uuid,
    ) -> AppResult<(Option<DateTime<FixedOffset>>, u64)> {
        let user = entity::user::Entity::find_by_id(user_id)
            .one(&self.database)
            .await?
            .ok_or(AppError::UserNotFound)?;

        let seconds_remaining = match user.last_airdrop {
            Some(last) => {
                let next_allowed = last + AIRDROP_COOLDOWN;
                let now: DateTime<FixedOffset> = Utc::now().into();
                let remaining = next_allowed - now;
                if remaining.num_seconds() > 0 {
                    remaining.num_seconds() as u64
                } else {
                    0
                }
            }
            None => 0,
        };

        Ok((user.last_airdrop, seconds_remaining))
    }

    pub async fn request_airdrop(&self, user_id: Uuid, cents: u64) -> AppResult<()> {
        let user = entity::user::Entity::find_by_id(user_id)
            .one(&self.database)
            .await?
            .ok_or(AppError::UserNotFound)?;

        if let Some(last_airdrop) = user.last_airdrop {
            let next_allowed = last_airdrop + AIRDROP_COOLDOWN;
            if Utc::now() < next_allowed {
                return Err(AppError::AirdropCooldown);
            }
        }

        let wallet = Keypair::from_base58_string(&user.wallet);

        let current_balance = match self.get_balance_in_cents(user_id).await {
            Ok(balance) => balance,
            Err(e) => {
                if e.to_string().contains("AccountNotFound") {
                    0
                } else {
                    return Err(e);
                }
            }
        };

        // x 10k to make it into micro usd
        self.solana.airdrop_usdc(&wallet.pubkey(), (current_balance + cents) * 10000).await?;

        let mut active_user: entity::user::ActiveModel = user.into();
        active_user.last_airdrop = Set(Some(Utc::now().into()));
        active_user.update(&self.database).await?;

        Ok(())
    }
}
