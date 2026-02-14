use blockchain_client::ProfeciaClient;
use blockchain_core::instructions::{FakeCancelOrderArgs, FakeCreateOrderArgs};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
};
use uuid::Uuid;

use crate::{
    AppState, entity,
    error::{AppError, AppResult},
    state::event::MarketOptionDto,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyOrderDto {
    pub id: Uuid,
    pub market_id: Uuid,
    pub user_id: Uuid,
    pub shares: i64,
    pub price_per_share: i64,
    pub option: MarketOptionDto,
}

impl From<entity::buyorder::Model> for BuyOrderDto {
    fn from(model: entity::buyorder::Model) -> Self {
        BuyOrderDto {
            id: model.id,
            market_id: model.market_id,
            user_id: model.user_id,
            shares: model.shares,
            price_per_share: model.price_per_share,
            option: MarketOptionDto::from(model.option),
        }
    }
}

impl AppState {
    pub async fn create_buy_order(
        &self,
        market_id: Uuid,
        user_id: Uuid,
        shares: i64,
        cents_per_share: i64,
        option: MarketOptionDto,
    ) -> AppResult<()> {
        let transaction = self.database.begin().await?;

        let market = entity::market::Entity::find_by_id(market_id)
            .one(&transaction)
            .await?
            .ok_or(AppError::MarketNotFound)?;

        let user = entity::user::Entity::find_by_id(user_id)
            .one(&transaction)
            .await?
            .ok_or(AppError::UserNotFound)?;

        if market.resolved_option.is_some() {
            return Err(AppError::MarketAlreadyResolved);
        }

        let event_id = market.event_id;
        // let event_pda = blockchain_client::ProfeciaClient::derive_event_pubkey(&event_id);
        // tracing::error!("event pda: {}", event_pda);

        // check balance
        let usdc_per_share = cents_per_share * 10000;
        let user_wallet = Keypair::from_base58_string(user.wallet.as_str());

        let user_ata_amount = match self.solana.fetch_usdc(&user_wallet.pubkey()).await {
            Ok(user_ata) => user_ata.amount,
            Err(e) => {
                // Account doesn't exist => balance is 0
                if e.to_string().contains("AccountNotFound") {
                    0
                } else {
                    return Err(e.into());
                }
            }
        };
        let necessary_usdc: u64 = (usdc_per_share * shares).try_into().unwrap();

        if necessary_usdc >= user_ata_amount {
            return Err(AppError::InsufficientFunds);
        }

        let option = match option {
            MarketOptionDto::OptionA => entity::market::MarketOption::A,
            MarketOptionDto::OptionB => entity::market::MarketOption::B,
        };

        let opposing_option = option.opposite();
        let opposing_price = 100 - cents_per_share;

        // Find opposing buy orders that match (price_per_share == 100 - our price)
        let opposing_orders = entity::buyorder::Entity::find()
            .filter(entity::buyorder::Column::MarketId.eq(market.id))
            .filter(entity::buyorder::Column::Option.eq(opposing_option.clone()))
            .filter(entity::buyorder::Column::PricePerShare.eq(opposing_price))
            .filter(entity::buyorder::Column::Shares.gt(0))
            .order_by_asc(entity::buyorder::Column::CreatedAt)
            .all(&transaction)
            .await?;

        let mut my_remaining = shares;

        for opposing in opposing_orders {
            if my_remaining == 0 {
                break;
            }

            let matched_qty = my_remaining.min(opposing.shares);

            // Create position for the new order's user (gets shares of their chosen option)
            AppState::upsert_position(
                &transaction,
                market.id,
                user_id,
                option.clone(),
                matched_qty,
                cents_per_share,
            )
            .await?;

            // Create position for the opposing order's user (gets shares of opposing option)
            AppState::upsert_position(
                &transaction,
                market.id,
                opposing.user_id,
                opposing_option.clone(),
                matched_qty,
                opposing.price_per_share,
            )
            .await?;

            // Update or delete the opposing order
            let new_opposing_shares = opposing.shares - matched_qty;
            let opposing_id = opposing.id;

            if new_opposing_shares == 0 {
                entity::buyorder::Entity::delete_by_id(opposing_id)
                    .exec(&transaction)
                    .await?;
            } else {
                let mut opposing_active: entity::buyorder::ActiveModel = opposing.into();
                opposing_active.shares = Set(new_opposing_shares);
                opposing_active.update(&transaction).await?;
            }

            my_remaining -= matched_qty;
        }

        if my_remaining > 0 {
            entity::buyorder::ActiveModel {
                id: Set(Uuid::new_v4()),
                market_id: Set(market.id),
                user_id: Set(user_id),
                option: Set(option),
                shares: Set(my_remaining),
                price_per_share: Set(cents_per_share),
                created_at: Set(Utc::now().into()),
            }
            .insert(&transaction)
            .await?;
        }

        transaction.commit().await?;

        // blockchain tx to transfer funds
        let create_order_args = FakeCreateOrderArgs {
            event_uuid: event_id,
            option_uuid: market_id,
            num_shares: shares.try_into().unwrap(),
            price_per_share: usdc_per_share.try_into().unwrap(),
        };
        let _sig = self.solana
            .create_order(&user_wallet, &create_order_args)
            .await?;

        Ok(())
    }

    pub async fn cancel_buy_order(
        txn: &impl sea_orm::ConnectionTrait,
        order_id: Uuid,
        num_shares: i64,
        cents_per_share: i64,
        event_id: Uuid,
        user_pubkey: &Pubkey,
        solana: &ProfeciaClient,
    ) -> AppResult<Signature> {
        entity::buyorder::Entity::delete_by_id(order_id)
            .exec(txn)
            .await?;

        let cents_per_share: u64 = cents_per_share.try_into().unwrap();

        let cancel_order_args = FakeCancelOrderArgs {
            event_uuid: event_id,
            option_uuid: order_id,
            num_shares: num_shares.try_into().unwrap(),
            price_per_share: cents_per_share * 10000,
        };
        let sig = solana.cancel_order(user_pubkey, &cancel_order_args).await?;

        Ok(sig)
    }

    pub async fn get_buy_order(
        txn: &impl sea_orm::ConnectionTrait,
        id: Uuid,
    ) -> AppResult<Option<BuyOrderDto>> {
        Ok(entity::buyorder::Entity::find_by_id(id)
            .one(txn)
            .await
            .map_err(Into::<AppError>::into)?
            .map(Into::into))
    }

    pub async fn list_buy_orders(&self, market_id: Uuid) -> AppResult<Vec<BuyOrderDto>> {
        Ok(entity::buyorder::Entity::find()
            .filter(entity::buyorder::Column::MarketId.eq(market_id))
            .all(&self.database)
            .await
            .map_err(Into::<AppError>::into)?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}
