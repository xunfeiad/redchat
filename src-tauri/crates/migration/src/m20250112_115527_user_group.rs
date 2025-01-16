use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::{ActiveValue::NotSet, EntityTrait, Set};
use entity::user_group;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserGroup::Table)
                    .if_not_exists()
                    .col(pk_auto(UserGroup::Id).comment("用户群组ID"))
                    .col(integer(UserGroup::UserId).comment("用户ID"))
                    .col(integer(UserGroup::GroupId).comment("群组ID"))
                    .col(
                        boolean(UserGroup::IsDeleted)
                            .default(false)
                            .comment("是否删除"),
                    )
                    .col(
                        timestamp(UserGroup::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp(UserGroup::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_user_group_user_group")
                            .col(UserGroup::UserId)
                            .col(UserGroup::GroupId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_group_user")
                            .from(UserGroup::Table, UserGroup::UserId)
                            .to(
                                super::m20250112_114132_user::User::Table,
                                super::m20250112_114132_user::User::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_group_group")
                            .from(UserGroup::Table, UserGroup::GroupId)
                            .to(
                                super::m20250112_115520_group::Group::Table,
                                super::m20250112_115520_group::Group::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        let user_groups = vec![
            user_group::ActiveModel {
                id: NotSet,
                user_id: Set(1),
                group_id: Set(1),
                is_deleted: Set(false),
                created_at: Set(chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap()),
                updated_at: Set(chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap()),
            },
            user_group::ActiveModel {
                id: NotSet,
                user_id: Set(2),
                group_id: Set(1),
                is_deleted: Set(false),
                created_at: Set(chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap()),
                updated_at: Set(chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap()),
            },
        ];
        entity::user_group::Entity::insert_many(user_groups)
            .exec(manager.get_connection())
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserGroup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserGroup {
    Table,
    Id,
    UserId,
    GroupId,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}
