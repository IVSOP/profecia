use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(has_one)]
    pub identity: HasOne<super::identity::Entity>,
    #[sea_orm(has_many)]
    pub sessions: HasMany<super::session::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
