use entity::user::ActiveModel;
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id).comment("用户ID"))
                    .col(string(User::Username).unique_key().comment("用户名"))
                    .col(string(User::Password).unique_key().comment("密码"))
                    .col(string_null(User::Avatar).comment("头像"))
                    .col(string_null(User::Nickname).comment("昵称"))
                    .col(string_null(User::Phone).comment("手机号"))
                    .col(string_null(User::Email).unique_key())
                    .col(string_null(User::Gender).comment("性别"))
                    .col(string_null(User::Birthday).comment("生日"))
                    .col(string_null(User::Address).comment("地址"))
                    .col(string_null(User::Signature).comment("签名"))
                    .col(boolean(User::Online).default(false).comment("是否在线"))
                    .col(boolean(User::IsDeleted).default(false).comment("是否删除"))
                    .col(
                        timestamp(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .col(
                        timestamp(User::LastLoginAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .comment("最后登录时间"),
                    )
                    .col(integer(User::Status).default(0).comment("状态"))
                    .to_owned(),
            )
            .await?;
        let db = manager.get_connection();

        let user1 = ActiveModel {
            id: Set(1),
            username: Set("xunfei".to_string()),
            password: Set(
                "f9566788443daf0670d86a9fc1f3ac5019cb27c502c257f3a0f2815e6f0a7d46".to_string(),
            ),
            avatar: Set(None),
            nickname: Set(Some("小飞".to_string())),
            phone: Set(None),
            email: Set("xunfei@126.com".to_string()),
            gender: Set(None),
            birthday: Set(None),
            address: Set(None),
            signature: Set(None),
            online: Set(false),
            is_deleted: Set(false),
            created_at: Set(
                chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap(),
            ),
            updated_at: Set(
                chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap(),
            ),
            last_login_at: Set(
                chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.004Z").unwrap(),
            ),
            status: Set(0),
        };

        let user2 = ActiveModel {
            id: Set(2),
            username: Set("hongdou".to_string()),
            password: Set(
                "73f531a71f5071cbcac6717627c2aec4a94098b7cfc6c95f8dcd96bf35ee3b01".to_string(),
            ),
            avatar: Set(None),
            nickname: Set(Some("豆子".to_string())),
            phone: Set(None),
            email: Set("hongdou@126.com".to_string()),
            gender: Set(None),
            birthday: Set(None),
            address: Set(None),
            signature: Set(None),
            online: Set(false),
            is_deleted: Set(false),
            created_at: Set(
                chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap(),
            ),
            updated_at: Set(
                chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap(),
            ),
            last_login_at: Set(
                chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.004Z").unwrap(),
            ),
            status: Set(0),
        };

        // 插入数据
        entity::user::Entity::insert_many(vec![user1, user2])
            .exec(db)
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Username,
    Password,
    Avatar,
    Nickname,
    Phone,
    Email,
    Online,
    // 0: 正常 1: 禁用
    Status,
    Gender,
    Birthday,
    Address,
    Signature,
    CreatedAt,
    UpdatedAt,
    LastLoginAt,
    IsDeleted,
}
