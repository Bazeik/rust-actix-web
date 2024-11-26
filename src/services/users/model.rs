use sea_orm::{entity::prelude::*, Set};
use chrono::{NaiveDateTime, Utc};
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub full_name: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C: ConnectionTrait>(
        mut self,
        _: &C,
        insert: bool,
    ) -> Result<Self, DbErr> {
        let mut active_model = self;
        let now = Utc::now().naive_utc();

        if insert {
            active_model.created_at = Set(now);
        }
        active_model.updated_at = Set(now);

        Ok(active_model)
    }
}
