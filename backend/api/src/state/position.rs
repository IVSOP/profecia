use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QuerySelect,
    RelationTrait, sea_query::JoinType,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    entity::{self},
    error::AppResult,
    state::event::MarketOptionDto,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDto {
    pub id: Uuid,
    pub market_id: Uuid,
    pub user_id: Uuid,
    pub option: MarketOptionDto,
    pub shares: i64,
    pub price_per_share: i64,
}

impl From<entity::position::Model> for PositionDto {
    fn from(model: entity::position::Model) -> Self {
        let option = MarketOptionDto::from(model.option);
        Self {
            id: model.id,
            market_id: model.market_id,
            user_id: model.user_id,
            option,
            shares: model.shares,
            price_per_share: model.price_per_share,
        }
    }
}

impl AppState {
    pub async fn list_user_positions_in_event(
        &self,
        user_id: Uuid,
        event_id: Uuid,
    ) -> AppResult<Vec<PositionDto>> {
        let positions = entity::position::Entity::find()
            .join(
                JoinType::InnerJoin,
                entity::position::Relation::Market.def(),
            )
            .filter(entity::position::Column::UserId.eq(user_id))
            .filter(entity::market::Column::EventId.eq(event_id))
            .all(&self.database)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(positions)
    }

    pub async fn list_user_positions(&self, user_id: Uuid) -> AppResult<Vec<PositionDto>> {
        let positions = entity::position::Entity::find()
            .filter(entity::position::Column::UserId.eq(user_id))
            .all(&self.database)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(positions)
    }

    /// Upsert a position: if the user already holds shares for this market+option+price, add to
    /// them; otherwise create a new position row.
    pub async fn upsert_position(
        txn: &impl sea_orm::ConnectionTrait,
        market_id: Uuid,
        user_id: Uuid,
        option: entity::market::MarketOption,
        shares: i64,
        price_per_share: i64,
    ) -> AppResult<()> {
        let existing = entity::position::Entity::find()
            .filter(entity::position::Column::MarketId.eq(market_id))
            .filter(entity::position::Column::UserId.eq(user_id))
            .filter(entity::position::Column::Option.eq(option.clone()))
            .filter(entity::position::Column::PricePerShare.eq(price_per_share))
            .one(txn)
            .await?;

        match existing {
            Some(position) => {
                let new_shares = position.shares + shares;
                let mut active: entity::position::ActiveModel = position.into();
                active.shares = Set(new_shares);
                active.update(txn).await?;
            }
            None => {
                entity::position::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    market_id: Set(market_id),
                    user_id: Set(user_id),
                    option: Set(option),
                    shares: Set(shares),
                    price_per_share: Set(price_per_share),
                }
                .insert(txn)
                .await?;
            }
        }

        Ok(())
    }
}
