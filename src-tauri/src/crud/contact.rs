use crate::schema::chat::{Contact, Message};
use crate::TauriState;
use entity::message::{ActiveModel,Column, Entity};
use error::{Response, Result};
use sea_orm::{DbBackend, FromQueryResult, QueryOrder};
use sea_orm::{prelude::Expr, ActiveModelTrait, Set, EntityTrait, QueryFilter, Statement, ColumnTrait};
use tauri::{command, State};
/**
 * pub struct Message {
    pub id: u64,
    pub name: String,
    pub avatar: String,
    // pub lastMessage: String,
    pub unread: u64,
    pub online: bool
}
 */

#[command]
pub async fn get_contacts(
    state: State<'_, TauriState>,
    user_id: i32,
) -> Result<Response<Vec<Contact>>> {
    let contact = Contact::find_by_statement(Statement::from_sql_and_values(DbBackend::Postgres,format!(
        r#"
            SELECT 
            "contact".friend_id AS id,
            "contact".name AS name,
            "contact".avatar AS avatar,
            COUNT(case when chat."message".status = 0 then 1 else null end) AS unread ,
            "contact".online AS online,
            MAX(chat."message".created_at) AS last_message_time, -- 取得最后一条消息的时间
            (
            SELECT msg.content
            FROM chat."message" AS msg
            WHERE msg.sender = "contact".friend_id AND msg.receiver = 1
            OR msg.sender = 1 AND msg.receiver = "contact".friend_id
            ORDER BY msg.created_at DESC
            LIMIT 1
            ) AS last_message -- 取得最后一条消息的内容
            FROM chat."message"
            JOIN (
            SELECT 
            chat.user_friend.friend_id AS friend_id, 
            chat."user".nickname AS name, 
            chat."user".avatar AS avatar,
            chat."user".online AS online
            FROM chat.user_friend
            JOIN chat."user"
            ON chat.user_friend.friend_id = chat."user".id
            WHERE chat.user_friend.user_id = {}
            AND chat.user_friend.status = 2
            ) AS "contact"
            ON chat."message".sender = "contact".friend_id
            WHERE chat."message".receiver = {}
            GROUP BY 
            "contact".friend_id, 
            "contact".name, 
            "contact".avatar, 
            "contact".online;
    "#,
        user_id, user_id
    ),[])).all(&state.db).await?;
    println!("{:?}", contact);
    Ok(Response::success(contact))
}

#[command]
pub async fn clear_unread(
    state: State<'_, TauriState>,
    user_id: i32,
    contact_id: i32,
) -> Result<Response<()>> {
    let contacts = Entity::update_many()
    .col_expr(Column::Status, Expr::value(1))
    .filter(Column::Receiver.eq(user_id))
    .filter(Column::Sender.eq(contact_id))
    .exec(&state.db)
    .await?;
    println!("{:?}", contacts);
    Ok(Response::success(()))
}

#[command]
pub async fn get_messages(state: State<'_, TauriState>, user_id: i32, contact_id: i32) -> Result<Response<Vec<Message>>> {
    let messages = Entity::find().filter(Column::Receiver.is_in([user_id, contact_id])).filter(Column::Sender.is_in([user_id, contact_id])).order_by_asc(Column::CreatedAt).all(&state.db).await?;
    let message: Vec<Message> = messages.into_iter().map(|m| Message {
        id: m.id,
        content: m.content,
        // TODO: Only text for now
        category: String::from("text"),
        is_self: m.sender == user_id,
        timestamp: m.created_at,
        status: String::from("sent"),
    }).collect();
    Ok(Response::success(message))
}

#[command]
pub async fn send_message(state: State<'_, TauriState>, user_id: i32, contact_id: i32, message: String) -> Result<Response<()>> {
    println!("send_message: {:?}", message);
    let message = ActiveModel {
        sender: Set(user_id),
        receiver: Set(contact_id),
        content: Set(message),
        ..Default::default()
    };
    println!("{:?}", message);
    message.insert(&state.db).await?;
    Ok(Response::success(()))
}