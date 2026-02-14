use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use uuid::Uuid;

use crate::{AppState, entity, error::AppResult, state::user::UserDto};

impl AppState {
    const SESSION_DURATION: chrono::Duration = chrono::Duration::days(7);

    pub async fn create_session(&self, user_id: Uuid) -> AppResult<Uuid> {
        let session_id = Uuid::new_v4();
        let created_at = Utc::now().fixed_offset();
        let expires_at = (Utc::now() + Self::SESSION_DURATION).fixed_offset();

        let session = entity::session::ActiveModel {
            id: Set(session_id),
            user_id: Set(user_id),
            created_at: Set(created_at),
            expires_at: Set(expires_at),
        };

        session.insert(&self.database).await?;
        Ok(session_id)
    }

    pub async fn delete_session(&self, session_id: Uuid) -> AppResult<()> {
        entity::session::Entity::delete_by_id(session_id)
            .exec(&self.database)
            .await?;
        Ok(())
    }

    pub async fn get_user_by_session_id(&self, session_id: Uuid) -> AppResult<Option<UserDto>> {
        let Some(session) = entity::session::Entity::find_by_id(session_id)
            .one(&self.database)
            .await?
        else {
            return Ok(None);
        };

        if session.expires_at.timestamp() <= Utc::now().timestamp() {
            let _ = self.delete_session(session_id).await;
            return Ok(None);
        }

        Ok(self.get_user_by_id(session.user_id).await?)
    }
}
