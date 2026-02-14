use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, entity, error::AppResult};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDto {
    pub id: Uuid,
    pub display_name: String,
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

impl From<(entity::event::Model, Vec<entity::market::Model>)> for EventDto {
    fn from((event, markets): (entity::event::Model, Vec<entity::market::Model>)) -> Self {
        EventDto {
            id: event.id,
            display_name: event.display_name,
            markets: markets.into_iter().map(Into::into).collect(),
        }
    }
}

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

impl AppState {
    pub async fn get_all_events(&self) -> AppResult<Vec<EventDto>> {
        let rows = entity::event::Entity::find()
            .find_with_related(entity::market::Entity)
            .all(&self.database)
            .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn get_event_by_id(&self, id: Uuid) -> AppResult<Option<EventDto>> {
        let result = entity::event::Entity::find_by_id(id)
            .find_with_related(entity::market::Entity)
            .all(&self.database)
            .await?
            .into_iter()
            .next()
            .map(Into::into);

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

        let mut markets = Vec::with_capacity(event.markets.len());

        for market in event.markets {
            let market_display_name = market.display_name.trim().to_string();

            let market = entity::market::ActiveModel {
                id: Set(Uuid::new_v4()),
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

        let event_dto = EventDto {
            id: event_id,
            display_name: event.display_name,
            markets,
        };

        Ok(event_dto)
    }
}
