use entity::message::ActiveModel;
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(pk_auto(Message::Id).comment("ID"))
                    .col(string(Message::Content).comment("消息内容"))
                    .col(integer(Message::Sender).comment("发送人ID"))
                    .col(integer(Message::Receiver).comment("接收ID"))
                    .col(integer(Message::ChatType).default(1).comment("聊天类型"))
                    .col(integer(Message::Status).default(0).comment("状态"))
                    .col(
                        boolean(Message::IsDeleted)
                            .default(false)
                            .comment("是否删除"),
                    )
                    .col(
                        timestamp(Message::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp(Message::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_message_sender")
                            .from(Message::Table, Message::Sender)
                            .to(
                                super::m20250112_114132_user::User::Table,
                                super::m20250112_114132_user::User::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_message_receiver")
                            .from(Message::Table, Message::Receiver)
                            .to(
                                super::m20250112_114132_user::User::Table,
                                super::m20250112_114132_user::User::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let messages = vec![
            ActiveModel {
                id: Set(1),
                content: Set("你好，请问是红豆小姐吗".to_string()),
                sender: Set(1),
                receiver: Set(2),
                chat_type: Set(1),
                status: Set(0),
                is_deleted: Set(false),
                created_at: Set(
                    chrono::DateTime::parse_from_rfc3339("2025-01-13T17:14:21.998Z").unwrap(),
                ),
                updated_at: Set(
                    chrono::DateTime::parse_from_rfc3339("2025-01-13T17:14:21.998Z").unwrap(),
                ),
            },
            ActiveModel {
                id: Set(4),
                content: Set("在的".to_string()),
                sender: Set(1),
                receiver: Set(2),
                chat_type: Set(1),
                status: Set(0),
                is_deleted: Set(false),
                created_at: Set(
                    chrono::DateTime::parse_from_rfc3339("2025-01-13T23:50:25.679Z").unwrap(),
                ),
                updated_at: Set(
                    chrono::DateTime::parse_from_rfc3339("2025-01-13T23:50:25.679Z").unwrap(),
                ),
            },
            ActiveModel {
                id: Set(5),
                content: Set("我在的".to_string()),
                sender: Set(1),
                receiver: Set(2),
                chat_type: Set(1),
                status: Set(0),
                is_deleted: Set(false),
                created_at: Set(
                    chrono::DateTime::parse_from_rfc3339("2025-01-13T23:52:56.723Z").unwrap(),
                ),
                updated_at: Set(
                    chrono::DateTime::parse_from_rfc3339("2025-01-13T23:52:56.723Z").unwrap(),
                ),
            },
            ActiveModel {
                id: Set(6),
                content: Set("你最近过的怎么样？".to_string()),
                sender: Set(1),
                receiver: Set(2),
                chat_type: Set(1),
                status: Set(0),
                is_deleted: Set(false),
                created_at: Set(
                    chrono::DateTime::parse_from_rfc3339("2025-01-13T23:54:33.733Z").unwrap(),
                ),
                updated_at: Set(
                    chrono::DateTime::parse_from_rfc3339("2025-01-13T23:54:33.733Z").unwrap(),
                ),
            },
            ActiveModel {
                id: Set(2),
                content: Set("是的，请问你是".to_string()),
                sender: Set(2),
                receiver: Set(1),
                chat_type: Set(1),
                status: Set(1),
                is_deleted: Set(false),
                created_at: Set(
                    chrono::DateTime::parse_from_rfc3339("2025-01-13T17:17:14.770Z").unwrap(),
                ),
                updated_at: Set(
                    chrono::DateTime::parse_from_rfc3339("2025-01-13T17:17:14.770Z").unwrap(),
                ),
            },
            ActiveModel {
                id: Set(3),
                content: Set("你好，还在吗".to_string()),
                sender: Set(2),
                receiver: Set(1),
                chat_type: Set(1),
                status: Set(1),
                is_deleted: Set(false),
                created_at: Set(
                    chrono::DateTime::parse_from_rfc3339("2025-01-13T17:30:42.177Z").unwrap(),
                ),
                updated_at: Set(
                    chrono::DateTime::parse_from_rfc3339("2025-01-13T17:30:42.177Z").unwrap(),
                ),
            },
        ];

        entity::message::Entity::insert_many(messages)
            .exec(manager.get_connection())
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Message {
    Table,
    Id,
    Sender,
    Receiver,
    Content,
    CreatedAt,
    UpdatedAt,
    // 1: 单聊 2: 群聊
    ChatType,
    // 1: 未读 2: 已读
    Status,
    // 是否删除
    IsDeleted,
}
