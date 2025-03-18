use axum::{Json, extract::{State, Path}};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use crate::middleware::AuthenticatedUser;
use crate::auth::{hash_senha, verificar_senha, gerar_token};

// ==========================
// Modelos para Usuários
// ==========================

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

// ==========================
// Funções de Usuário
// ==========================

/// Registra um novo usuário no sistema.
pub async fn register_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<NovoUsuario>,
) -> Result<Json<AuthResponse>, axum::http::StatusCode> {
    // Gera o hash da senha utilizando Argon2.
    let senha_hash = hash_senha(&payload.senha)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insere o novo usuário no banco de dados com o tipo "usuario".
    let result = sqlx::query!(
        "INSERT INTO usuarios (nome, email, senha_hash, tipo) VALUES (?, ?, ?, 'usuario')",
        payload.nome,
        payload.email,
        senha_hash
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(Json(AuthResponse {
            message: "Usuário cadastrado com sucesso!".to_string(),
            token: None,
        })),
        Err(_) => Err(axum::http::StatusCode::BAD_REQUEST),
    }
}

/// Realiza o login do usuário, retornando um token JWT em caso de sucesso.
pub async fn login_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, axum::http::StatusCode> {
    let user = sqlx::query_as!(
        crate::models::Usuario, // Certifique-se de que o modelo Usuario esteja definido em models.rs
        "SELECT * FROM usuarios WHERE email = ?",
        payload.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(user) = user {
        if verificar_senha(&payload.senha, &user.senha_hash).unwrap_or(false) {
            let token = gerar_token(&user.email);
            return Ok(Json(AuthResponse {
                message: "Login realizado com sucesso!".to_string(),
                token: Some(token),
            }));
        }
    }
    Err(axum::http::StatusCode::UNAUTHORIZED)
}

// ==========================
// Modelos para Tarefas
// ==========================

#[derive(Serialize, sqlx::FromRow)]
pub struct Tarefa {
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

// ==========================
// Funções de Tarefa
// ==========================

/// Cria uma nova tarefa para o usuário autenticado.
pub async fn create_task(
    State(pool): State<SqlitePool>,
    AuthenticatedUser(email): AuthenticatedUser,
    Json(payload): Json<NovaTarefa>,
) -> Json<String> {
    let user_id = sqlx::query!(
        "SELECT id FROM usuarios WHERE email = ?",
        email
    )
    .fetch_one(&pool)
    .await
    .expect("Usuário não encontrado")
    .id;

    sqlx::query!(
        "INSERT INTO tarefas (titulo, descricao, status, usuario_id) VALUES (?, ?, 'pendente', ?)",
        payload.titulo, payload.descricao, user_id
    )
    .execute(&pool)
    .await
    .expect("Erro ao inserir tarefa");

    Json("Tarefa criada com sucesso!".to_string())
}

/// Lista as tarefas do usuário autenticado.
pub async fn get_tasks(
    State(pool): State<SqlitePool>,
    AuthenticatedUser(email): AuthenticatedUser,
) -> Json<Vec<Tarefa>> {
    let user_id = sqlx::query!(
        "SELECT id FROM usuarios WHERE email = ?",
        email
    )
    .fetch_one(&pool)
    .await
    .expect("Usuário não encontrado")
    .id;

    let tarefas = sqlx::query_as!(
        Tarefa,
        "SELECT * FROM tarefas WHERE usuario_id = ?",
        user_id
    )
    .fetch_all(&pool)
    .await
    .expect("Erro ao buscar tarefas");

    Json(tarefas)
}

/// Atualiza uma tarefa do usuário autenticado.
pub async fn update_task(
    State(pool): State<SqlitePool>,
    AuthenticatedUser(email): AuthenticatedUser,
    Path(id): Path<i32>,
    Json(payload): Json<NovaTarefa>,
) -> Json<String> {
    let user_id = sqlx::query!(
        "SELECT id FROM usuarios WHERE email = ?",
        email
    )
    .fetch_one(&pool)
    .await
    .expect("Usuário não encontrado")
    .id;

    sqlx::query!(
        "UPDATE tarefas SET titulo = ?, descricao = ? WHERE id = ? AND usuario_id = ?",
        payload.titulo, payload.descricao, id, user_id
    )
    .execute(&pool)
    .await
    .expect("Erro ao atualizar tarefa");

    Json("Tarefa atualizada com sucesso!".to_string())
}

/// Deleta uma tarefa do usuário autenticado.
pub async fn delete_task(
    State(pool): State<SqlitePool>,
    AuthenticatedUser(email): AuthenticatedUser,
    Path(id): Path<i32>,
) -> Json<String> {
    let user_id = sqlx::query!(
        "SELECT id FROM usuarios WHERE email = ?",
        email
    )
    .fetch_one(&pool)
    .await
    .expect("Usuário não encontrado")
    .id;

    sqlx::query!(
        "DELETE FROM tarefas WHERE id = ? AND usuario_id = ?",
        id, user_id
    )
    .execute(&pool)
    .await
    .expect("Erro ao deletar tarefa");

    Json("Tarefa deletada com sucesso!".to_string())
}
