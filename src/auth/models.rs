use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SignUp {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct LogInResponse {
    pub token_type : String,
    pub access_token: String,
}

pub struct AuthenticatedUser {
    pub claims: Claims,
}
