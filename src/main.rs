use axum::{routing::get, Router};
use db::connect_db;
use std::net::SocketAddr;
use dotenv::dotenv;
use std::env;
use sqlx::SqlitePool; 

mod auth;
mod db;
mod models;
mod routes;
mod handlers;
mod middleware; 


#[tokio::main]

async fn main() {
    dotenv().ok();
    
    let pool = db::connect_db().await; //conecta ao banco de dados

    let app = routes::create_router(pool); //carrega as rotas

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor rodando em http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
