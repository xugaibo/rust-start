use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn get_connect() -> DatabaseConnection{
    let mut opt = ConnectOptions::new("mysql://root:123456@127.0.0.1/go_start".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await;
    match db {
        Ok(db) => {
            return db
        },
        Err(e) => {
            panic!("{}", e)
        }
    }
}