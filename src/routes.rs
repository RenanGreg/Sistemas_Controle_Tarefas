use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::SqlitePool;
use crate::handlers::{criar_livro, get_livro, update_livro, delete_livro};

pub fn create_router(db: SqlitePool) -> Router {
    Router::new()
        .route("/livros", post(create_livro).get(get_livro))
        .route("/livros/:id", put(update_livro).delete(delete_livro))
        .with_state(db)
}
