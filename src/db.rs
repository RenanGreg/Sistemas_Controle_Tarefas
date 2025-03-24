use dotenvy::dotenv;
use sqlx::SqlitePool;
use std::env;

pub async fn connect_db() -> SqlitePool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL não definida no .env");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Falha ao conectar ao banco de dados");

    sqlx::query(include_str!("../migrations/001_create_tables.sql"))
        .execute(&pool)
        .await
        .expect("Falha ao rodar a migration");

    pool
}
