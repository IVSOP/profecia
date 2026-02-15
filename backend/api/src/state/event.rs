use std::collections::HashMap;

use blockchain_client::ProfeciaClient;
use blockchain_core::{
    accounts::event::EventOption,
    instructions::{AddOptionArgs, CreateEmptyEventArgs, FakeGetRewardArgs},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ExprTrait, ModelTrait, QueryFilter,
    QuerySelect, TransactionTrait,
    sea_query::{Expr, Alias},
};
use serde::{Deserialize, Serialize};
use solana_sdk::{signature::Keypair, signer::Signer};
use uuid::Uuid;

use crate::{
    AppState,
    entity::{self, market::MarketOption},
    error::{AppError, AppResult},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDto {
    pub id: Uuid,
    pub display_name: String,
    pub image_url: Option<String>,
    pub solana_url: String,
    pub pubkey: String,
    pub markets: Vec<MarketDto>,
    pub pending_buy_orders: i64,
    pub volume: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDto {
    pub id: Uuid,
    pub display_name: String,
    pub image_url: Option<String>,
    pub option_a_name: String,
    pub option_b_name: String,
    pub rules: String,
    pub resolved_option: Option<MarketOptionDto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarketOptionDto {
    OptionA,
    OptionB,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventRequest {
    pub display_name: String,
    pub image_url: Option<String>,
    pub markets: Vec<MarketRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketRequest {
    pub display_name: String,
    pub image_url: Option<String>,
    pub option_a_name: String,
    pub option_b_name: String,
    pub rules: String,
}

// impl From<(entity::event::Model, Vec<entity::market::Model>)> for EventDto {
//     fn from((event, markets): (entity::event::Model, Vec<entity::market::Model>)) -> Self {
//         EventDto {
//             id: event.id,
//             display_name: event.display_name,
//             markets: markets.into_iter().map(Into::into).collect(),
//         }
//     }
// }

impl From<entity::market::Model> for MarketDto {
    fn from(value: entity::market::Model) -> Self {
        MarketDto {
            id: value.id,
            display_name: value.display_name,
            image_url: value.image_url,
            option_a_name: value.option_a_name,
            option_b_name: value.option_b_name,
            rules: value.rules,
            resolved_option: value.resolved_option.map(Into::into),
        }
    }
}

impl From<entity::market::MarketOption> for MarketOptionDto {
    fn from(value: entity::market::MarketOption) -> Self {
        match value {
            entity::market::MarketOption::A => MarketOptionDto::OptionA,
            entity::market::MarketOption::B => MarketOptionDto::OptionB,
        }
    }
}

impl From<MarketOptionDto> for entity::market::MarketOption {
    fn from(value: MarketOptionDto) -> Self {
        match value {
            MarketOptionDto::OptionA => entity::market::MarketOption::A,
            MarketOptionDto::OptionB => entity::market::MarketOption::B,
        }
    }
}

impl AppState {
    pub async fn get_all_events(&self) -> AppResult<Vec<EventDto>> {
        let rows = entity::event::Entity::find()
            .find_with_related(entity::market::Entity)
            .all(&self.database)
            .await?;

        // Collect all market IDs to count pending buy orders
        let all_market_ids: Vec<Uuid> = rows
            .iter()
            .flat_map(|(_, markets)| markets.iter().map(|m| m.id))
            .collect();

        // Count buy orders per market
        let market_order_counts: HashMap<Uuid, i64> = if all_market_ids.is_empty() {
            HashMap::new()
        } else {
            entity::buyorder::Entity::find()
                .filter(entity::buyorder::Column::MarketId.is_in(all_market_ids.clone()))
                .select_only()
                .column(entity::buyorder::Column::MarketId)
                .column_as(entity::buyorder::Column::Id.count(), "order_count")
                .group_by(entity::buyorder::Column::MarketId)
                .into_tuple::<(Uuid, i64)>()
                .all(&self.database)
                .await?
                .into_iter()
                .collect()
        };

        // Compute volume (total shares) per market from positions, then aggregate to events
        let market_volumes: HashMap<Uuid, i64> = if all_market_ids.is_empty() {
            HashMap::new()
        } else {
            entity::position::Entity::find()
                .filter(entity::position::Column::MarketId.is_in(all_market_ids))
                .select_only()
                .column(entity::position::Column::MarketId)
                .column_as(
                    Expr::col(entity::position::Column::Shares).sum().cast_as(Alias::new("BIGINT")),
                    "total_shares",
                )
                .group_by(entity::position::Column::MarketId)
                .into_tuple::<(Uuid, Option<i64>)>()
                .all(&self.database)
                .await?
                .into_iter()
                .map(|(id, sum)| (id, sum.unwrap_or(0)))
                .collect()
        };

        Ok(rows
            .into_iter()
            .map(|(event, markets)| {
                let event_pda = ProfeciaClient::derive_event_pubkey(&event.id);
                let pending: i64 = markets
                    .iter()
                    .map(|m| market_order_counts.get(&m.id).copied().unwrap_or(0))
                    .sum();
                let volume: i64 = markets
                    .iter()
                    .map(|m| market_volumes.get(&m.id).copied().unwrap_or(0))
                    .sum();
                EventDto {
                    id: event.id,
                    display_name: event.display_name,
                    image_url: event.image_url,
                    solana_url: self.solana.get_account_url(&event_pda),
                    pubkey: event_pda.to_string(),
                    markets: markets.into_iter().map(Into::into).collect(),
                    pending_buy_orders: pending,
                    volume,
                }
            })
            .collect())
    }

    pub async fn get_event_by_id(&self, id: Uuid) -> AppResult<Option<EventDto>> {
        let result = entity::event::Entity::find_by_id(id)
            .find_with_related(entity::market::Entity)
            .all(&self.database)
            .await?
            .into_iter()
            .next();

        let Some((event, markets)) = result else {
            return Ok(None);
        };

        let market_ids: Vec<Uuid> = markets.iter().map(|m| m.id).collect();
        let pending: i64 = if market_ids.is_empty() {
            0
        } else {
            entity::buyorder::Entity::find()
                .filter(entity::buyorder::Column::MarketId.is_in(market_ids.clone()))
                .select_only()
                .column_as(entity::buyorder::Column::Id.count(), "order_count")
                .into_tuple::<i64>()
                .one(&self.database)
                .await?
                .unwrap_or(0)
        };

        // Compute volume (total shares) for this event from positions
        let volume: i64 = if market_ids.is_empty() {
            0
        } else {
            entity::position::Entity::find()
                .filter(entity::position::Column::MarketId.is_in(market_ids))
                .select_only()
                .column_as(
                    Expr::col(entity::position::Column::Shares).sum().cast_as(Alias::new("BIGINT")),
                    "total_shares",
                )
                .into_tuple::<Option<i64>>()
                .one(&self.database)
                .await?
                .flatten()
                .unwrap_or(0)
        };

        let event_pda = ProfeciaClient::derive_event_pubkey(&event.id);
        Ok(Some(EventDto {
            id: event.id,
            display_name: event.display_name,
            image_url: event.image_url,
            solana_url: self.solana.get_account_url(&event_pda),
            pubkey: event_pda.to_string(),
            markets: markets.into_iter().map(Into::into).collect(),
            pending_buy_orders: pending,
            volume,
        }))
    }

    pub async fn create_event(&self, event: EventRequest) -> AppResult<EventDto> {
        let transaction = self.database.begin().await?;
        let event_id = Uuid::new_v4();
        let display_name = event.display_name.trim().to_string();
        let event_image_url = event
            .image_url
            .as_deref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        entity::event::ActiveModel {
            id: Set(event_id),
            display_name: Set(display_name),
            image_url: Set(event_image_url.clone()),
        }
        .insert(&transaction)
        .await?;

        let mut token_keypairs: Vec<(Keypair, Keypair)> = Vec::new();
        let mut markets_map: HashMap<Uuid, EventOption> = HashMap::new();

        let mut markets = Vec::with_capacity(event.markets.len());

        for market in event.markets {
            let market_display_name = market.display_name.trim().to_string();
            let market_image_url = market
                .image_url
                .as_deref()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());

            let yes_keypair = Keypair::new();
            let no_keypair = Keypair::new();
            let market_id = Uuid::new_v4();

            let yes_keypair_string = yes_keypair.to_base58_string();
            let no_keypair_string = no_keypair.to_base58_string();

            markets_map.insert(
                market_id,
                EventOption {
                    option_desc: "".into(),
                    yes_mint: yes_keypair.pubkey(),
                    no_mint: no_keypair.pubkey(),
                },
            );

            token_keypairs.push((yes_keypair, no_keypair));

            let market = entity::market::ActiveModel {
                id: Set(market_id),
                yes_keypair: Set(yes_keypair_string),
                no_keypair: Set(no_keypair_string),
                display_name: Set(market_display_name),
                image_url: Set(market_image_url.clone()),
                event_id: Set(event_id),
                option_a_name: Set(market.option_a_name),
                option_b_name: Set(market.option_b_name),
                rules: Set(market.rules),
                resolved_option: Set(None),
            };

            let market = market.insert(&transaction).await?;
            markets.push(MarketDto {
                id: market.id,
                display_name: market.display_name,
                image_url: market_image_url,
                option_a_name: market.option_a_name,
                option_b_name: market.option_b_name,
                rules: market.rules,
                resolved_option: None,
            });
        }

        transaction.commit().await?;

        // let create_event_args = CreateEventArgs {
        //     uuid: event_id,
        //     description: "".into(),
        //     options: markets_map,
        // };
        // let _sig = self
        //     .solana
        //     .create_event(&token_keypairs, &create_event_args)
        //     .await?;

        let create_empty_event_args = CreateEmptyEventArgs {
            uuid: event_id,
            description: "".into(),
        };
        let _sig = self
            .solana
            .create_empty_event(&create_empty_event_args)
            .await?;

        for ((option_id, option_info), (yes_token, no_token)) in
            markets_map.iter().zip(token_keypairs.iter())
        {
            let add_option_args = AddOptionArgs {
                event_uuid: event_id,
                option_uuid: *option_id,
                option_info: option_info.clone(),
            };
            let _sig = self
                .solana
                .add_option(yes_token, no_token, &add_option_args)
                .await?;
        }

        let event_pda = ProfeciaClient::derive_event_pubkey(&event_id);

        let event_dto = EventDto {
            id: event_id,
            solana_url: self.solana.get_account_url(&event_pda),
            display_name: event.display_name,
            image_url: event_image_url,
            pubkey: event_pda.to_string(),
            markets,
            pending_buy_orders: 0,
            volume: 0,
        };

        Ok(event_dto)
    }

    pub async fn resolve_market(&self, market_id: Uuid, option: MarketOptionDto) -> AppResult<Vec<String>> {
        let transaction = self.database.begin().await?;

        let market = entity::market::Entity::find_by_id(market_id)
            .one(&transaction)
            .await?
            .ok_or(AppError::MarketNotFound)?;

        if market.resolved_option.is_some() {
            return Err(AppError::MarketAlreadyResolved);
        }

        let winning_option: entity::market::MarketOption = option.into();
        let mut tx_urls: Vec<String> = Vec::new();

        // Cancel all remaining buy orders for this market
        let buy_orders = market
            .find_related(entity::buyorder::Entity)
            .all(&transaction)
            .await?;

        for order in &buy_orders {
            let user = entity::user::Entity::find_by_id(order.user_id)
                .one(&transaction)
                .await?
                .ok_or(AppError::UserNotFound)?;

            let user_wallet = Keypair::from_base58_string(&user.wallet);

            let sig = AppState::cancel_buy_order(
                &transaction,
                order.id,
                order.shares,
                order.price_per_share,
                market.event_id,
                &user_wallet.pubkey(),
                &self.solana,
            )
            .await?;
            tx_urls.push(self.solana.get_transaction_url(&sig));
        }

        // Pay out positions that hold the winning option
        let winning_positions = entity::position::Entity::find()
            .filter(entity::position::Column::MarketId.eq(market_id))
            .filter(entity::position::Column::Option.eq(winning_option.clone()))
            .all(&transaction)
            .await?;

        let winning_mint = match winning_option {
            MarketOption::A => Keypair::from_base58_string(market.yes_keypair.as_str()).pubkey(),
            MarketOption::B => Keypair::from_base58_string(market.no_keypair.as_str()).pubkey(),
        };

        for position in &winning_positions {
            // blockchain tx to get rewards for this position
            let fake_get_reward_args = FakeGetRewardArgs {
                event_uuid: market.event_id,
                option_uuid: market_id,
                num_shares: position.shares.try_into().unwrap(),
            };

            let user = entity::user::Entity::find_by_id(position.user_id)
                .one(&transaction)
                .await?
                .ok_or(AppError::UserNotFound)?;

            let user_wallet = Keypair::from_base58_string(&user.wallet);

            let sig = self
                .solana
                .get_reward(&user_wallet, &winning_mint, &fake_get_reward_args)
                .await?;
            tx_urls.push(self.solana.get_transaction_url(&sig));
        }

        // Mark the market as resolved
        let mut active_market: entity::market::ActiveModel = market.into();
        active_market.resolved_option = Set(Some(winning_option));
        active_market.update(&transaction).await?;

        transaction.commit().await?;

        Ok(tx_urls)
    }
}
