use std::collections::HashMap;

use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    entity::{self, market::MarketOption},
    error::AppResult,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardEntry {
    pub user_id: Uuid,
    pub username: String,
    pub realized_profit_cents: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardResponse {
    pub entries: Vec<LeaderboardEntry>,
}

impl AppState {
    pub async fn get_leaderboard(&self) -> AppResult<LeaderboardResponse> {
        let users = entity::user::Entity::find()
            .all(&self.database)
            .await?;

        let positions = entity::position::Entity::find()
            .find_also_related(entity::market::Entity)
            .all(&self.database)
            .await?;

        // Start with every user at zero profit (includes users with no shares)
        let mut user_profits: HashMap<Uuid, i64> =
            users.iter().map(|u| (u.id, 0)).collect();

        // Add realized profit from resolved positions
        for (position, market) in &positions {
            let Some(market) = market else { continue };
            let Some(resolved_option) = &market.resolved_option else { continue };
            let won = match (&position.option, resolved_option) {
                (MarketOption::A, MarketOption::A) => true,
                (MarketOption::B, MarketOption::B) => true,
                _ => false,
            };
            let payout = if won { position.shares * 100 } else { 0 };
            let cost = position.shares * position.price_per_share;
            if let Some(profit) = user_profits.get_mut(&position.user_id) {
                *profit += payout - cost;
            }
        }

        let mut entries: Vec<LeaderboardEntry> = users
            .iter()
            .map(|user| LeaderboardEntry {
                user_id: user.id,
                username: user.username.clone(),
                realized_profit_cents: user_profits[&user.id],
            })
            .collect();

        entries.sort_by(|a, b| b.realized_profit_cents.cmp(&a.realized_profit_cents));

        Ok(LeaderboardResponse { entries })
    }
}
