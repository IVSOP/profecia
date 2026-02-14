use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub display_name: String,
    #[sea_orm(has_many)]
    pub markets: HasMany<super::market::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
