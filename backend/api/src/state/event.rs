use std::collections::HashMap;

use blockchain_client::ProfeciaClient;
use blockchain_core::{accounts::event::EventOption, instructions::CreateEventArgs};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};
use solana_sdk::{signature::Keypair, signer::Signer};
use uuid::Uuid;

use crate::{
    AppState, entity,
    error::{AppError, AppResult},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDto {
    pub id: Uuid,
    pub display_name: String,
    pub url: String,
    pub markets: Vec<MarketDto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDto {
    pub id: Uuid,
    pub display_name: String,
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
    pub markets: Vec<MarketRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketRequest {
    pub display_name: String,
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

        Ok(rows
            .into_iter()
            .map(|(event, markets)| {
                let event_pda = ProfeciaClient::derive_event_pubkey(&event.id);
                EventDto {
                    id: event.id,
                    display_name: event.display_name,
                    url: self.solana.get_account_url(&event_pda),
                    markets: markets.into_iter().map(Into::into).collect(),
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
            .next()
            .map(|(event, markets)| {
                let event_pda = ProfeciaClient::derive_event_pubkey(&event.id);
                EventDto {
                    id: event.id,
                    display_name: event.display_name,
                    url: self.solana.get_account_url(&event_pda),
                    markets: markets.into_iter().map(Into::into).collect(),
                }
            });

        Ok(result)
    }

    pub async fn create_event(&self, event: EventRequest) -> AppResult<EventDto> {
        let transaction = self.database.begin().await?;
        let event_id = Uuid::new_v4();
        let display_name = event.display_name.trim().to_string();

        entity::event::ActiveModel {
            id: Set(event_id),
            display_name: Set(display_name),
        }
        .insert(&transaction)
        .await?;

        let mut token_keypairs: Vec<Keypair> = Vec::new();
        let mut markets_map: HashMap<Uuid, EventOption> = HashMap::new();

        let mut markets = Vec::with_capacity(event.markets.len());

        for market in event.markets {
            let market_display_name = market.display_name.trim().to_string();

            let yes_keypair = Keypair::new();
            let no_keypair = Keypair::new();
            let market_id = Uuid::new_v4();

            let yes_keypair_string = yes_keypair.to_base58_string();
            let no_keypair_string = no_keypair.to_base58_string();

            markets_map.insert(
                market_id,
                EventOption {
                    option_desc: "isto nao esta feito".into(),
                    yes_mint: yes_keypair.pubkey(),
                    no_mint: no_keypair.pubkey(),
                },
            );

            token_keypairs.push(yes_keypair);
            token_keypairs.push(no_keypair);

            let market = entity::market::ActiveModel {
                id: Set(market_id),
                yes_keypair: Set(yes_keypair_string),
                no_keypair: Set(no_keypair_string),
                display_name: Set(market_display_name),
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
                option_a_name: market.option_a_name,
                option_b_name: market.option_b_name,
                rules: market.rules,
                resolved_option: None,
            });
        }

        transaction.commit().await?;

        let create_event_args = CreateEventArgs {
            uuid: event_id,
            description: "isto nao esta feito".into(),
            options: markets_map,
        };
        let _sig = self
            .solana
            .create_event(&token_keypairs, &create_event_args)
            .await?;

        let event_pda = ProfeciaClient::derive_event_pubkey(&event_id);

        let event_dto = EventDto {
            id: event_id,
            url: self.solana.get_account_url(&event_pda),
            display_name: event.display_name,
            markets,
        };

        Ok(event_dto)
    }

    pub async fn resolve_market(&self, market_id: Uuid, option: MarketOptionDto) -> AppResult<()> {
        let transaction = self.database.begin().await?;

        let market = entity::market::Entity::find_by_id(market_id)
            .one(&transaction)
            .await?
            .ok_or(AppError::MarketNotFound)?;

        if market.resolved_option.is_some() {
            return Err(AppError::MarketAlreadyResolved);
        }

        let winning_option: entity::market::MarketOption = option.into();

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

            AppState::cancel_buy_order(
                &transaction,
                order.id,
                order.shares,
                order.price_per_share,
                market.event_id,
                &user_wallet.pubkey(),
                &self.solana,
            )
            .await?;
        }

        // Pay out positions that hold the winning option
        let winning_positions = entity::position::Entity::find()
            .filter(entity::position::Column::MarketId.eq(market_id))
            .filter(entity::position::Column::Option.eq(winning_option.clone()))
            .all(&transaction)
            .await?;

        for position in &winning_positions {
            // TODO: add position.shares to user balance (each share pays out 100)
            let _ = position;
        }

        // Mark the market as resolved
        let mut active_market: entity::market::ActiveModel = market.into();
        active_market.resolved_option = Set(Some(winning_option));
        active_market.update(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }
}
