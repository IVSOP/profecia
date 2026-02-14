use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "market_snapshot")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub market_id: Uuid,
    pub option_a_percentage: Option<i64>,
    pub option_b_percentage: Option<i64>,
    pub recorded_at: DateTimeWithTimeZone,
    #[sea_orm(belongs_to, from = "market_id", to = "id")]
    pub market: HasOne<super::market::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
