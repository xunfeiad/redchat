//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use chrono::{FixedOffset, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(schema_name = "chat", table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    #[sea_orm(unique)]
    pub password: String,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    #[sea_orm(unique)]
    pub phone: Option<String>,
    #[sea_orm(unique)]
    pub email: String,
    pub gender: Option<String>,
    pub birthday: Option<String>,
    pub address: Option<String>,
    pub signature: Option<String>,
    pub online: bool,
    pub is_deleted: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub last_login_at: DateTimeWithTimeZone,
    pub status: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        println!("before_save");
        let offset: FixedOffset = FixedOffset::east_opt(8 * 60 * 60).unwrap();
        let now_with_offset = Utc::now().with_timezone(&offset);
        if insert {
            self.created_at = ActiveValue::Set(now_with_offset);
            if self.updated_at.clone().into_value().is_none() {
                self.updated_at = ActiveValue::Set(now_with_offset);
            }
            self.status = ActiveValue::NotSet;
        } else {
            self.updated_at = ActiveValue::Set(now_with_offset);
        }
        Ok(self)
    }
}
