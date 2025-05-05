use actix::fut::result;
use actix_web::{web, HttpResponse, Responder};
use argon2::password_hash::{rand_core, SaltString};
use serde::de::IntoDeserializer;
use sqlx::{PgPool, Row};
use crate::AppState;
use crate::auth::hash::{hash_pwd_salted, verify_pwd_salted};
use crate::auth::models::{LogInResponse, SignUp, User};
use crate::routes::models::ApiResponse;
use crate::auth::jwt::{generate_jwt, validate_jwt};

pub async fn sign_up(
    data: web::Data<AppState>,
    payload: web::Json<SignUp>
)-> impl Responder {

    let user_check = sqlx::query("SELECT * FROM Users WHERE username = $1")
        .bind(&payload.username)
        .fetch_one(&data.db_pool)
        .await;

    if let Err(sqlx::Error::RowNotFound) = user_check {
        
        let salted_hashed_pwd = hash_pwd_salted(&SaltString::generate(&mut rand_core::OsRng), &payload.password).expect("panic message");
        
        let result = sqlx::query("insert into users (username, password) values ($1, $2)")
            .bind(&payload.username)
            .bind(&salted_hashed_pwd)
            .execute(&data.db_pool)
            .await;

        return match result {
            Ok(result) => HttpResponse::Ok().body("User Created Successfully"),
            Err(_) => HttpResponse::InternalServerError().body("Failed to create user")
        }
    }

    HttpResponse::Conflict().body("User already exist with the name.")
}


pub async fn log_in(
    payload: web::Json<SignUp>,
    data: web::Data<AppState>
) -> impl Responder {
    let user_result = sqlx::query_as::<_, User>("SELECT * FROM Users WHERE username = $1")
        .bind(&payload.username)
        .fetch_one(&data.db_pool)
        .await;

    match user_result {
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().body("User does not exist.")
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("DB error: {}", e))
        }
        Ok(user) => {
            let password_verified =
                verify_pwd_salted(&payload.password, &user.password).unwrap_or(false);

            if password_verified {
                let auth_token = generate_jwt(&user.id.to_string());

                if let Ok(token) = auth_token {
                    HttpResponse::Ok().json(ApiResponse {
                        status: "Success".to_string(),
                        msg: "User logged in successfully".to_string(),
                        data: LogInResponse {
                            token_type : "Bearer".to_string(),
                            access_token: token
                        },
                    })
                } else {
                    HttpResponse::InternalServerError().body("Token generation failed.")
                }
            } else {
                HttpResponse::Unauthorized().json(ApiResponse {
                    status: "Failure".to_string(),
                    msg: "Incorrect password".to_string(),
                    data: "Invalid".to_string(),
                })
            }
        }
    }
}

