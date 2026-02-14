use std::collections::HashMap;

use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, entity, error::AppResult};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketSnapshotPointDto {
    pub recorded_at: String,
    pub percentages: HashMap<Uuid, Option<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventChartDto {
    pub points: Vec<MarketSnapshotPointDto>,
}

impl AppState {
    /// Record a snapshot of the current percentages for all markets.
    pub async fn record_market_snapshots(&self) -> AppResult<()> {
        let now = Utc::now();

        // Get all markets with their buy orders to compute percentages
        let rows = entity::market::Entity::find()
            .find_with_related(entity::buyorder::Entity)
            .all(&self.database)
            .await?;

        for (market, buy_orders) in &rows {
            let (option_a_pct, option_b_pct) = implied_probability(buy_orders);

            entity::market_snapshot::ActiveModel {
                id: Set(Uuid::new_v4()),
                market_id: Set(market.id),
                option_a_percentage: Set(option_a_pct),
                option_b_percentage: Set(option_b_pct),
                recorded_at: Set(now.into()),
            }
            .insert(&self.database)
            .await?;
        }

        Ok(())
    }

    /// Get the percentage history for all markets in an event.
    pub async fn get_event_chart(&self, event_id: Uuid) -> AppResult<EventChartDto> {
        // First get the market ids for this event
        let markets = entity::market::Entity::find()
            .filter(entity::market::Column::EventId.eq(event_id))
            .all(&self.database)
            .await?;

        let market_ids: Vec<Uuid> = markets.iter().map(|m| m.id).collect();

        if market_ids.is_empty() {
            return Ok(EventChartDto { points: vec![] });
        }

        // Fetch all snapshots for these markets, ordered by time
        let snapshots = entity::market_snapshot::Entity::find()
            .filter(entity::market_snapshot::Column::MarketId.is_in(market_ids.clone()))
            .order_by(entity::market_snapshot::Column::RecordedAt, Order::Asc)
            .all(&self.database)
            .await?;

        // Group snapshots by recorded_at timestamp
        let mut time_groups: Vec<(String, HashMap<Uuid, Option<i64>>)> = Vec::new();

        for snapshot in snapshots {
            let time_key = snapshot.recorded_at.to_rfc3339();

            if let Some(last) = time_groups.last_mut() {
                if last.0 == time_key {
                    last.1
                        .insert(snapshot.market_id, snapshot.option_a_percentage);
                    continue;
                }
            }

            let mut map = HashMap::new();
            map.insert(snapshot.market_id, snapshot.option_a_percentage);
            time_groups.push((time_key, map));
        }

        let points = time_groups
            .into_iter()
            .map(|(recorded_at, percentages)| MarketSnapshotPointDto {
                recorded_at,
                percentages,
            })
            .collect();

        Ok(EventChartDto { points })
    }
}

/// Compute implied probability from buy orders (reused logic from market state).
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
