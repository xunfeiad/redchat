use crate::schema::user::{UserLogin, UserRegister};
use crate::TauriState;
use deadpool_redis::redis::cmd;
use entity::user::Column as UserColumn;
use entity::user::Entity as User;
use entity::user::{ActiveModel, Model};
use error::{Error, Response, Result};
use sea_orm::entity::{ActiveModelTrait, ColumnTrait, EntityTrait};
use sea_orm::QueryFilter;
use tauri::command;
use tauri::State;
use utils::email::SendEmail;

#[command]
pub async fn login(state: State<'_, TauriState>, user_login: UserLogin) -> Result<Response<Model>> {
    let user = user_login.hash_password();
    let user = User::find()
        .filter(UserColumn::Username.eq(user.username))
        .filter(UserColumn::Password.eq(user.password))
        .one(&state.db)
        .await?
        .ok_or(Error::UsernameOrPasswordIncorrect)?;
    Ok(Response::success(user))
}

#[command]
pub async fn register(
    state: State<'_, TauriState>,
    user_register: UserRegister,
) -> Result<Response<u8>, Error> {
    // let mut redis = state.redis_pool.get().await?;
    // let code = cmd("GET").arg(&[user_register.email.as_str()]).query_async::<String>(&mut redis).await?;
    // if code != user_register.code {
    //     return Err(Error::EmailCodeIncorrect);
    // }
    let user: ActiveModel = user_register.into_active_model();
    user.insert(&state.db).await?;
    Ok(Response::success(1))
}

#[command]
pub async fn send_email_code(state: State<'_, TauriState>, email: String) -> Result<Response<u8>> {
    let user = User::find()
        .filter(UserColumn::Email.eq(&email))
        .one(&state.db)
        .await?;
    if user.is_some() {
        return Err(Error::EmailAlreadyExists);
    }
    let random_str = utils::get_random_str();
    let mut redis = state.redis_pool.get().await?;
    cmd("SET")
        .arg(&[email.as_str(), random_str.as_str()])
        .query_async::<()>(&mut redis)
        .await?;
    let mut context = tera::Context::new();
    context.insert("code", &random_str);
    let body = state.tera.render("verify_code.html", &context)?;
    body.send_email("Varift Email", email)?;
    Ok(Response::success(1))
}

#[command]
pub async fn verify_email_code(
    state: State<'_, TauriState>,
    email: String,
    input_code: String,
) -> Result<Response<u8>> {
    let mut redis = state.redis_pool.get().await?;
    let code = cmd("GET")
        .arg(&[email.as_str()])
        .query_async::<String>(&mut redis)
        .await?;
    if code == input_code {
        Ok(Response::success(1))
    } else {
        Err(Error::EmailCodeIncorrect)
    }
}
