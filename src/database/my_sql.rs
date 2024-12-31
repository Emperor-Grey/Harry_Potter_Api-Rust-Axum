use std::time::Duration;

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub async fn connect_database(url: &str) -> MySqlPool {
    MySqlPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_millis(5000))
        .connect(url)
        .await
        .expect("Failed to connect to the database")
}
