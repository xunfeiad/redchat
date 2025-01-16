use sea_orm_migration::{prelude::*, schema::*};
use entity::group;
use sea_orm::{ActiveValue::NotSet, EntityTrait, Set};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(pk_auto(Group::Id).comment("群组ID"))
                    .col(string(Group::Name).comment("群组名称"))
                    .col(string(Group::Description).comment("群组描述"))
                    .col(integer(Group::OwnerId).comment("群主ID"))
                    .col(integer(Group::Status).default(0).comment("状态"))
                    .col(string_null(Group::Avatar).comment("头像"))
                    .col(string(Group::InviteCode).comment("邀请码"))
                    .col(boolean(Group::IsDeleted).default(false).comment("是否删除"))
                    .col(
                        timestamp(Group::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp(Group::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_group_owner")
                            .from(Group::Table, Group::OwnerId)
                            .to(
                                super::m20250112_114132_user::User::Table,
                                super::m20250112_114132_user::User::Id,
                            )
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;
        let groups = vec![
            group::ActiveModel {
                id: NotSet,
                name: Set("家人群".to_string()),
                description: Set("我们是一家人".to_string()),
                owner_id: Set(1),
                status: Set(0),
                avatar: Set(None),
                invite_code: Set("1212".to_string()),
                is_deleted: Set(false),
                created_at: Set(chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap()),
                updated_at: Set(chrono::DateTime::parse_from_rfc3339("2025-01-13T17:02:18.001Z").unwrap()),
            },
        ];
        entity::group::Entity::insert_many(groups)
            .exec(manager.get_connection())
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Group {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
    OwnerId,
    // 0: 正常 1: 禁用
    Status,
    Avatar,
    InviteCode,
    IsDeleted,
}
