use dotenvy::dotenv;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;

pub async fn connect_db() -> sqlx::SqlitePool {
    let database_url = "sqlite://database.db";
    let pool = sqlx::SqlitePool::connect(database_url)
        .await
        .expect("Falha ao conectar ao banco de dados");

    // Executa a migration
    sqlx::query(include_str!("../migrations/001_create_tables.sql"))
        .execute(&pool)
        .await
        .expect("Falha ao rodar a migration");

    pool
}
