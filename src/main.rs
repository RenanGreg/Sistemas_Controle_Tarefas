use axum::Router;
use std::net::SocketAddr;
mod db;
mod models;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    // Conecta ao banco e executa as migrations
    let pool = db::connect_db().await;

    // Cria o roteador com as rotas definidas
    let app = routes::create_router(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor rodando em http://127.0.0.1:3000{}", addr); 

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
