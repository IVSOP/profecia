use std::collections::HashMap;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, entity, error::AppResult};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketPercentagesDto {
    pub option_a_percentage: Option<i64>,
    pub option_b_percentage: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventPercentagesDto {
    pub percentages: HashMap<Uuid, MarketPercentagesDto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllPercentagesDto {
    pub percentages: HashMap<Uuid, EventPercentagesDto>,
}

impl AppState {
    /// Compute implied probabilities for all markets across all events.
    pub async fn get_all_percentages(&self) -> AppResult<AllPercentagesDto> {
        let rows = entity::market::Entity::find()
            .find_with_related(entity::buyorder::Entity)
            .all(&self.database)
            .await?;

        let percentages = rows
            .iter()
            .fold(HashMap::new(), |mut acc, (market, buy_orders)| {
                let (id, dto) = market_percentages(market, buy_orders);
                acc.entry(market.event_id)
                    .or_insert_with(|| EventPercentagesDto {
                        percentages: HashMap::new(),
                    })
                    .percentages
                    .insert(id, dto);
                acc
            });

        Ok(AllPercentagesDto { percentages })
    }

    /// Compute implied probabilities for all markets in an event.
    pub async fn get_event_percentages(&self, event_id: Uuid) -> AppResult<EventPercentagesDto> {
        let rows = entity::market::Entity::find()
            .filter(entity::market::Column::EventId.eq(event_id))
            .find_with_related(entity::buyorder::Entity)
            .all(&self.database)
            .await?;

        let percentages = rows
            .iter()
            .map(|(market, buy_orders)| market_percentages(market, buy_orders))
            .collect();

        Ok(EventPercentagesDto { percentages })
    }
}

fn market_percentages(
    market: &entity::market::Model,
    buy_orders: &[entity::buyorder::Model],
) -> (Uuid, MarketPercentagesDto) {
    let (option_a_percentage, option_b_percentage) = implied_probability(buy_orders);
    (
        market.id,
        MarketPercentagesDto {
            option_a_percentage,
            option_b_percentage,
        },
    )
}

fn implied_probability(buy_orders: &[entity::buyorder::Model]) -> (Option<i64>, Option<i64>) {
    let mut best_a_bid: i64 = 0;
    let mut best_b_bid: i64 = 0;

    for order in buy_orders {
        match order.option {
            entity::market::MarketOption::A => {
                if order.price_per_share > best_a_bid {
                    best_a_bid = order.price_per_share;
                }
            }
            entity::market::MarketOption::B => {
                if order.price_per_share > best_b_bid {
                    best_b_bid = order.price_per_share;
                }
            }
        }
    }

    let pct_a = if best_a_bid > 0 && best_b_bid > 0 {
        let implied_from_a = best_a_bid;
        let implied_from_b = 100 - best_b_bid;
        (implied_from_a + implied_from_b + 1) / 2
    } else if best_a_bid > 0 {
        best_a_bid
    } else if best_b_bid > 0 {
        100 - best_b_bid
    } else {
        return (None, None);
    };

    (Some(pct_a), Some(100 - pct_a))
}
