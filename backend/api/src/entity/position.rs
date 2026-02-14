use sea_orm::entity::prelude::*;

use crate::entity::market::MarketOption;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "position")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub market_id: Uuid,
    pub user_id: Uuid,
    pub option: MarketOption,
    pub shares: i64,
    #[sea_orm(belongs_to, from = "market_id", to = "id")]
    pub market: HasOne<super::market::Entity>,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
