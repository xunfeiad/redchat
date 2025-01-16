use actix_ws::Session;
use bytestring::ByteString;
use crate::{Message, AppState};
use sea_orm::{ActiveValue::Set, ActiveModelTrait, EntityTrait};
use error::{Error as CusError, Response, Result};
use anyhow::Context;
use entity::user::{self, ActiveModel, Entity as User};
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
                            user.online = Set(true);
                            user.update(&state.db).await?;
                            session.text(serde_json::to_string(&Response::success_with_string("认证成功")).unwrap()).await.context("认证失败")?;
                            state.user_connections.find_session_and_send_message(None, &Message::Auth(auth_message), Some(user_id), Some(state.db.clone())).await?;
                        },
                        // todo
                        Message::Text(text_message) => {
                            if !state.user_connections.is_auth(user_id).await {
                                return Err(CusError::NoAuthorization);
                            }
                            let text_meesage_ref = &Message::Text(text_message.clone());
                            if text_message.group_id.is_some() {
                                let group = entity::user_group::Entity::find_by_id(
                                    text_message.group_id.unwrap(),
                                )
                                .all(&state.db)
                                .await?;
                                let user_ids =
                                    group.iter().map(|g| g.user_id).collect::<Vec<i32>>();
                                for user_id in user_ids {
                                    state.user_connections.find_session_and_send_message(Some(user_id), text_meesage_ref, None, None).await?;
                                }
                            } else {
                                let user_id = text_message.receiver_id.context("接收者ID为空")?;
                                println!("user_id: {}, session_ids: {:?}", user_id, state.user_connections.get_session_ids().await);
                                state.user_connections.find_session_and_send_message(Some(user_id), text_meesage_ref, None, None).await?;
                            }
                        },
                        Message::WebRTC(webrtc_message) => {
                            if !state.user_connections.is_auth(user_id).await {
                                return Err(CusError::NoAuthorization);
                            }
                            // todo
                            if webrtc_message.group_id.is_some() {
                                let group = entity::user_group::Entity::find_by_id(
                                    webrtc_message.group_id.unwrap(),
                                )
                                .all(&state.db)
                                .await?;
                                let user_ids =
                                    group.iter().map(|g| g.user_id).collect::<Vec<i32>>();
                                let webrtc_message_ref = &Message::WebRTC(webrtc_message);
                                for user_id in user_ids {
                                    state.user_connections.find_session_and_send_message(Some(user_id), webrtc_message_ref, None, None).await?;
                                }
                            } else {
                                let receiver_id = webrtc_message.receiver_id.context("接收者ID为空")?;
                                state.user_connections.find_session_and_send_message(Some(receiver_id), &Message::WebRTC(webrtc_message), None, None).await?;
                            }
                        },
                        _ =>{}
                    }
                    Ok(())
                    
}
