use sea_orm::{ActiveValue::Set, EntityTrait, NotSet};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserFriend::Table)
                    .if_not_exists()
                    .col(pk_auto(UserFriend::Id).comment("用户好友ID"))
                    .col(integer(UserFriend::UserId).comment("用户ID"))
                    .col(integer(UserFriend::FriendId).comment("好友ID"))
                    .col(integer(UserFriend::Status).default(0))
                    .col(boolean(UserFriend::IsDeleted).comment("是否删除"))
                    .col(
                        timestamp(UserFriend::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp(UserFriend::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_friend_user")
                            .from(UserFriend::Table, UserFriend::UserId)
                            .to(
                                super::m20250112_114132_user::User::Table,
                                super::m20250112_114132_user::User::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_friend_friend")
                            .from(UserFriend::Table, UserFriend::FriendId)
                            .to(
                                super::m20250112_114132_user::User::Table,
                                super::m20250112_114132_user::User::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        let friend1 = entity::user_friend::ActiveModel {
            id: NotSet,
            user_id: Set(1),
            friend_id: Set(2),
            status: Set(2),
            is_deleted: Set(false),
            created_at: Set(chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap()),
            updated_at: Set(chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap()),
        };
        let friend2 = entity::user_friend::ActiveModel {
            id: NotSet,
            user_id: Set(2),
            friend_id: Set(1),
            status: Set(2),
            is_deleted: Set(false),
            created_at: Set(
                chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap(),
            ),
            updated_at: Set(
                chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap(),
            ),
        };
        entity::user_friend::Entity::insert_many(vec![friend1, friend2])
            .exec(manager.get_connection())
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserFriend::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserFriend {
    Table,
    Id,
    UserId,
    FriendId,
    // 1: 待处理 2: 已处理 3: 已拒绝, 4: 黑名单
    Status,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}
