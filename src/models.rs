use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub tipo: String,
}

#[derive(Deserialize)]
pub struct NovoUsuario {
    pub nome: String,
    pub email: String,
    pub senha: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub senha: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub token: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Tarefas {
    pub id: i32,
    pub titulo: String,
    pub descricao: String,
    pub status: String,
    pub usuario_id: i32,
}

#[derive(Deserialize)]
pub struct NovaTarefa {
    pub titulo: String,
    pub descricao: String,
}
