use actix_ws::Session;
use bytestring::ByteString;
use crate::{Message, AppState};
use sea_orm::{ActiveValue::Set, ActiveModelTrait, EntityTrait};
use error::{Error as CusError, Response, Result};
use anyhow::Context;
use entity::user::{Entity as User,ActiveModel};
pub async fn handle_message(text: ByteString, state: &AppState, mut session: Session, user_id: i32) -> Result<(), CusError> {
    let message_data: Message = serde_json::from_str(&text)?;
                    match message_data {
                        Message::Auth(auth_message) => {
                            state
                                .user_connections
                                .add_session(auth_message.user_id, session.clone())
                                .await;
                            println!("认证成功: {:?}, session_ids: {:?}", auth_message.user_id, state.user_connections.get_session_ids().await);
                            let user = User::find_by_id(auth_message.user_id).one(&state.db).await?.ok_or(CusError::CustomError("用户不存在"))?;
                            let mut user: ActiveModel = user.into();
                            user.status = Set(1);
                            user.update(&state.db).await?;
                            session.text(serde_json::to_string(&Response::success("认证成功")).unwrap()).await.context("认证失败")?;
                        }
                        // todo
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
                                    {
                                        session.text(text.clone()).await.context("发送群消息失败")?;
                                    }
                                }
                            } else {
                                let user_id = text_message.receiver_id.context("接收者ID为空")?;
                                println!("user_id: {}, session_ids: {:?}", user_id, state.user_connections.get_session_ids().await);
                                if let Ok(ref mut session) = state
                                    .user_connections
                                    .get_session(user_id)
                                    .await
                                {
                                    session.text(text.clone()).await.context("发送私聊消息失败")?;
                                    println!("session found");
                                }else{
                                    println!("session not found");
                                    session.text("该用户不在线").await.context("发送私聊消息失败")?;
                                }
                            }
                        }
                        Message::WebRTC(mut webrtc_message) => {
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
                                    {
                                        session.text(text.clone()).await.context("发送群消息失败")?;
                                    }
                                }
                            } else {
                                let user_id = webrtc_message.receiver_id.context("接收者ID为空")?;
                                if let Ok(ref mut session) = state
                                    .user_connections
                                    .get_session(user_id)
                                    .await
                                {
                                    let user_info = entity::user::Entity::find_by_id(user_id).one(&state.db).await?.context("用户不存在")?;
                                    webrtc_message.sender_name = Some(user_info.username);
                                    session.text(serde_json::to_string(&webrtc_message).unwrap()).await.context("发送私聊消息失败")?;
                                    println!("session found: user_id: {}", user_id);
                                }else{
                                    println!("session not found: user_id: {}", user_id);
                                    session.text(Response::error_with_string("该用户不在线")).await.context("发送sdp消息失败")?;
                                }
                            }
                        }
                    }
    Ok(())
}
