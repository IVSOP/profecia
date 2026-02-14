use sea_orm::EntityTrait;
use solana_sdk::{signature::Keypair, signer::Signer};
use uuid::Uuid;

use crate::{
    AppState, entity,
    error::{AppError, AppResult},
};

impl AppState {
    pub async fn get_balance_in_cents(&self, user_id: Uuid) -> AppResult<u64> {
        let user = entity::user::Entity::find_by_id(user_id)
            .one(&self.database)
            .await?
            .ok_or(AppError::UserNotFound)?;

        let wallet = Keypair::from_base58_string(&user.wallet);

        let usdc = self.solana.fetch_usdc(&wallet.pubkey()).await?;

        return Ok(usdc.amount / 10000);
    }
}
