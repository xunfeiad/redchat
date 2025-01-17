use error::Result;
use sea_orm::Database;
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::time::Duration;


pub async fn get_db() -> Result<DatabaseConnection> {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    println!("database_url: {}", database_url);
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("chat"); // Setting default PostgreSQL schema

        let db = Database::connect(opt)
        .await?;
    Ok(db)
}


