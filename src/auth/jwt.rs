use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::auth::models::{AuthenticatedUser, Claims};
use std::env;
use std::future::{ready, Ready};
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;

pub fn generate_jwt(user_id: &str) -> jsonwebtoken::errors::Result<String> {
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = chrono::Utc::now().checked_add_signed(chrono::Duration::days(1)).unwrap().timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
}

pub fn validate_jwt(token: &str) -> jsonwebtoken::errors::Result<Claims> {
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret_key.as_ref()), &Validation::default());
    Ok(token_data?.claims)
}



impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        if let Some(header_value) = auth_header {
            if let Ok(auth_str) = header_value.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    match validate_jwt(token) {
                        Ok(claims) => {
                            return ready(Ok(AuthenticatedUser { claims }));
                        }
                        Err(_) => {
                            return ready(Err(actix_web::error::ErrorUnauthorized("Invalid JWT")));
                        }
                    }
                }
            }
        }

        ready(Err(actix_web::error::ErrorUnauthorized("Authorization header missing or invalid")))
    }
}
