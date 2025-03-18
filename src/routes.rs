 use axum::{
     routing::{get, post, put, delete},
     Router,
 };
 use crate::handlers::{ 
     register_user, 
     login_user, 
     create_task, 
     get_tasks, 
     update_task, 
     delete_task 
 };


pub fn create_router(db: sqlx::SqlitePool) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/tasks", post(create_task).get(get_tasks))
        .route("/tasks/:id", put(update_task).delete(delete_task))
        .with_state(db)
}
