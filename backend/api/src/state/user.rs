use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState, entity,
    error::{AppError, AppResult},
    utils::password::{hash_password, verify_password},
};

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

        let user = entity::user::ActiveModel {
            id: Set(user_id),
            username: Set(username.to_string()),
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
}
