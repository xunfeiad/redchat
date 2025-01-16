use actix_ws::Session;
use bytestring::ByteString;
use crate::{Message, AppState};
use sea_orm::prelude::*;
use error::{Error as CusError, Result};
use anyhow::Context;
pub async fn handle_message(text: ByteString, state: &AppState, mut session: Session, user_id: i32) -> Result<(), CusError> {
    let message_data: Message = serde_json::from_str(&text)?;
                    match message_data {
                        Message::Auth(auth_message) => {
                            state
                                .user_connections
                                .add_session(auth_message.user_id, session.clone())
                                .await;
                            session.text("认证成功").await.context("认证失败")?;
                        }
                        Message::Text(text_message) => {
                            if !state.user_connections.is_auth(user_id).await {
                                return Err(CusError::NoAuthorization);
                            }
                            if text_message.group_id.is_some() {
                                let group = entity::user_group::Entity::find_by_id(
                                    text_message.group_id.unwrap(),
                                )
                                .all(&state.db)
                                .await?;
                                let user_ids =
                                    group.iter().map(|g| g.user_id).collect::<Vec<i32>>();

                                for user_id in user_ids {
                                    if let Ok(ref mut session) = state
                                        .user_connections
                                        .get_session(user_id)
                                        .await
                                        .ok_or(CusError::SessionNotFound)
                                    {
                                        session.text(text.clone()).await.context("发送群消息失败")?;
                                    }
                                }
                            } else {
                                let user_id = text_message.receiver_id.context("接收者ID为空")?;
                                println!("user_id: {}", user_id);
                                if let Ok(ref mut session) = state
                                    .user_connections
                                    .get_session(user_id)
                                    .await
                                    .ok_or(CusError::SessionNotFound)
                                {
                                    session.text(text.clone()).await.context("发送私聊消息失败")?;
                                    println!("session found");
                                }else{
                                    println!("session not found");
                                    session.text("该用户不在线").await.context("发送私聊消息失败")?;
                                }
                            }
                        }
                        Message::WebRTC(webrtc_message) => {
                            if !state.user_connections.is_auth(user_id).await {
                                return Err(CusError::NoAuthorization);
                            }
                            if webrtc_message.group_id.is_some() {
                                let group = entity::user_group::Entity::find_by_id(
                                    webrtc_message.group_id.unwrap(),
                                )
                                .all(&state.db)
                                .await?;
                                let user_ids =
                                    group.iter().map(|g| g.user_id).collect::<Vec<i32>>();

                                for user_id in user_ids {
                                    if let Ok(ref mut session) = state
                                        .user_connections
                                        .get_session(user_id)
                                        .await
                                        .ok_or(CusError::SessionNotFound)
                                    {
                                        session.text(text.clone()).await.context("发送群消息失败")?;
                                    }
                                }
                            } else {
                                let user_id = webrtc_message.receiver_id.context("接收者ID为空")?;
                                println!("user_id: {}", user_id);
                                if let Ok(ref mut session) = state
                                    .user_connections
                                    .get_session(user_id)
                                    .await
                                    .ok_or(CusError::SessionNotFound)
                                {
                                    session.text(text.clone()).await.context("发送sdp消息失败")?;
                                    println!("session found");
                                }else{
                                    println!("session not found");
                                    session.text("该用户不在线").await.context("发送sdp消息失败")?;
                                }
                            }
                        }
                    }
    Ok(())
}