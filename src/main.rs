use axum::{Router, routing::get}; 
use std::net::SocketAddr; 

mod routes;
mod db; 
mod auth; 
mod models; 

#[tokio::main] 

async fn main() {
  let app = Router::new()
    .route("/", get(|| async { "Sistema de Controle de Tarefas" })); 

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); 
  println!("Servidor rodando em http://{}",addr); 

  axum::Server::bind(&addr); 
  .Serve(app.into_make_service()) 
  .await
  .unwrap(); 
  
}