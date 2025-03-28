use serde::{Deserialize, Serialize}; 

#[derive(Serialize, sqlx::FromRow)] 
pub struct Livro{
  pub id: i32,
  pub titulo: String,
  pub autor: Option<String>, 
  pub publicação: Option<String>,  
  pub genero:String,
} 

#[derive(Deserialize)]
pub struct NovoLivro{
  pub autor: Option<String>,
  pub publicação: Option<String>,
  pub genero: String,
} 
