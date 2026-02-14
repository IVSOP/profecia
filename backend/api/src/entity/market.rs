use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MarketOption {
    #[sea_orm(string_value = "option_a")]
    A,
    #[sea_orm(string_value = "option_b")]
    B,
}

impl MarketOption {
    pub fn opposite(&self) -> Self {
        match self {
            MarketOption::A => MarketOption::B,
            MarketOption::B => MarketOption::A,
        }
    }
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "market")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub yes_keypair: String,
    #[sea_orm(unique)]
    pub no_keypair: String,
    pub display_name: String,
    pub event_id: Uuid,
    pub option_a_name: String,
    pub option_b_name: String,
    pub rules: String,
    pub resolved_option: Option<MarketOption>,
    #[sea_orm(belongs_to, from = "event_id", to = "id")]
    pub event: HasOne<super::event::Entity>,
    #[sea_orm(has_many)]
    pub buy_orders: HasMany<super::buyorder::Entity>,
    #[sea_orm(has_many)]
    pub positions: HasMany<super::position::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
