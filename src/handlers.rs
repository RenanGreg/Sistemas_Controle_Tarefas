use axum::{Json, extract::{State, Path}};
use sqlx::SqlitePool;
use crate::models::{Livro, NovoLivro};

/// Cria um novo quadrinho e retorna o registro criado.
pub async fn create_comic(
    State(pool): State<SqlitePool>,
    Json(payload): Json<NovoLivro>,
) -> Json<Livro> {
    sqlx::query!(
        "INSERT INTO comics (titulo, autor, publicação, genero)
         VALUES (?, ?, ?, ?,)",
         payload.titulo,
         payload.autor,
         payload.publicação,
         payload.genero,
    )
    .execute(&pool)
    .await
    .expect("Erro ao inserir o livro");

    // Recupera o id do último registro inserido
    let comic_id = sqlx::query!("SELECT last_insert_rowid() as id")
        .fetch_one(&pool)
        .await
        .expect("Erro ao recuperar id")
        .id;

    let livro = sqlx::query_as!(
        Livro,
        "SELECT * FROM livros WHERE id = ?",
        livro_id
    )
    .fetch_one(&pool)
    .await
    .expect("Erro ao buscar livro");

    Json(livro)
}

/// Retorna a lista de todos os quadrinhos.
pub async fn get_comics(
    State(pool): State<SqlitePool>,
) -> Json<Vec<Livro>> {
    let livro = sqlx::query_as!(
        Livro,
        "SELECT * FROM livros"
    )
    .fetch_all(&pool)
    .await
    .expect("Erro ao buscar livros");

    Json(livros)
}

/// Atualiza um quadrinho existente com base no ID e nos dados fornecidos.
pub async fn update_comic(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
    Json(payload): Json<NovoLivro>,
) -> Json<Livro> {
    sqlx::query!(
        "UPDATE comics SET titulo = ?, autor = ?, publicação = ?, genero = ?, WHERE id = ?",
        payload.titulo,
        payload.autor,
        payload.publicação,
        payload.genero,
        id
    )
    .execute(&pool)
    .await
    .expect("Erro ao atualizar livro");

    let livro = sqlx::query_as!(
        Livro,
        "SELECT * FROM livro WHERE id = ?",
        id
    )
    .fetch_one(&pool)
    .await
    .expect("Erro ao buscar livro");

    Json(livro)
}

/// Deleta um quadrinho pelo ID.
pub async fn delete_comic(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Json<String> {
    sqlx::query!(
        "DELETE FROM livros WHERE id = ?",
        id
    )
    .execute(&pool)
    .await
    .expect("Erro ao deletar livro");

    Json(format!("Comic com id {} removida", id))
}
