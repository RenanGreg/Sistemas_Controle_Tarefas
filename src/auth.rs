use argon2::password_hash::{Error, PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn hash_senha(senha: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(senha.as_bytes(), &salt)?.to_string();
    Ok(hash)
}

pub fn verificar_senha(senha: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(senha.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn gerar_token(email: &str) -> String {
    let chave_secreta = env::var("JWT_SECRET").unwrap_or_else(|_| "minha_chave".into());
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::hours(2))
        .expect("Falha ao calcular expiração")
        .timestamp() as usize;

    let claims = Claims {
        sub: email.to_string(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(chave_secreta.as_ref()),
    )
    .unwrap()
}
