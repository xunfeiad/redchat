use deadpool_redis::redis::cmd;
use deadpool_redis::{Config, Pool, Runtime};
use error::Error;
use sea_orm::DatabaseConnection;
use tera::Tera;

pub struct TauriState {
    pub tera: Tera,
    pub db: DatabaseConnection,
    pub redis_pool: Pool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Error> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    dotenv::dotenv().ok();
    env_logger::builder().init();
    let redis_url = std::env::var("REDIS_URL").unwrap();
    runtime.block_on(async {
        let db = db::get_db().await.expect("error while connecting to database");
        let tera = Tera::new("templates/*.html").expect("can not find template.");
        let files = tera.get_template_names();
        log::error!("{:?}", files.collect::<Vec<_>>());
        let cfg = Config::from_url(redis_url);
        let pool = cfg
            .create_pool(Some(Runtime::Tokio1))
            .expect("error while creating redis pool");
        let mut conn = pool
            .get()
            .await
            .expect("error while getting redis connection");
        let ping = cmd("PING");
        assert_eq!(ping.query_async::<String>(&mut conn).await.unwrap(), "PONG");

        tauri::Builder::default()
            .manage(TauriState {
                tera: tera.clone(),
                db,
                redis_pool: pool,
            })
            .plugin(tauri_plugin_opener::init())
            .invoke_handler(tauri::generate_handler![
                crud::user::auth::login,
                crud::user::auth::send_email_code,
                crud::user::auth::register,
                crud::contact::get_contacts,
                crud::contact::clear_unread,
                crud::contact::get_messages,
                crud::contact::send_message
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });

    Ok(())
}

pub mod crud;
pub mod schema;
