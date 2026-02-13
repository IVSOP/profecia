use anyhow::Context;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

pub async fn hash_password(password: String) -> anyhow::Result<String> {
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let Ok(password) = argon2.hash_password(password.as_bytes(), &salt) else {
            anyhow::bail!("Failed to hash password");
        };

        Ok(password.to_string())
    })
    .await
    .context("Failed to run hash passowrd task")?
}

pub async fn verify_password(
    password: String,
    stored_password_hash: String,
) -> anyhow::Result<bool> {
    tokio::task::spawn_blocking(move || {
        let parsed_hash = match PasswordHash::new(&stored_password_hash) {
            Ok(hash) => hash,
            Err(_) => return false,
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    })
    .await
    .context("Failen to run verification password task")
}
