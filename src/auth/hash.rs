use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString , Error, Salt};
use crate::auth::models::User;

pub fn hash_pwd_salted(salt: &SaltString, pwd: &str) -> Result<String, Error> {
    let argon2 = Argon2::default();
    let hash_pwd = argon2.hash_password(pwd.as_bytes(), salt)?.to_string();
    Ok(hash_pwd)
}

pub fn verify_pwd_salted(raw_pwd: &str, hashed_pwd: &str) -> Result<bool, Error> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hashed_pwd).unwrap();
    let is_valid_pwd = argon2.verify_password(raw_pwd.as_ref(), &parsed_hash).is_ok();
    Ok(is_valid_pwd)
}