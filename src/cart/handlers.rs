use std::fs::exists;
use actix_web::{web, HttpResponse, Responder};
use actix_web::error::PayloadError::Http2Payload;
use chrono::Month::April;
use sqlx::PgPool;
use sqlx::types::Json;
use uuid::Uuid;
use crate::AppState;
use crate::auth::models::AuthenticatedUser;
use crate::cart::models::{AddItem, Cart, CartItem};
use crate::routes::models::ApiResponse;

pub async fn create_cart(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {

    let user_id = match Uuid::parse_str(&user.claims.sub) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid user ID format"),
    };

    let existing_cart = sqlx::query("SELECT * FROM shopping_cart WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(&data.db_pool)
        .await;

    match existing_cart {
        Ok(Some(_)) => HttpResponse::Ok().json(ApiResponse {
            status : "Conflict".to_string(),
            msg : "Cart Already Exist".to_string(),
            data : "No Data".to_string()
        }),
        Ok(None) => {
            let insert_result = sqlx::query("INSERT INTO shopping_cart(user_id) VALUES ($1)")
                .bind(&user_id)
                .execute(&data.db_pool)
                .await;

            match insert_result {
                Ok(_) => HttpResponse::Ok().json(ApiResponse {
                    status : "Success".to_string(),
                    msg : "Cart Created Successfully".to_string(),
                    data: "{}".to_string()
                }),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}


pub async fn get_cart_items(
    data: web::Data<AppState>,
    user: AuthenticatedUser
) -> impl Responder {

    let user_id = match Uuid::parse_str(&user.claims.sub) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid user ID format"),
    };
    
    let cart = sqlx::query_as::<_, Cart>("SELECT * FROM shopping_cart WHERE user_id = $1")
        .bind(&user_id)
        .fetch_optional(&data.db_pool)
        .await;

    match cart {
        Ok(None) => HttpResponse::Ok().json( ApiResponse { status : "Error".to_string(),msg: "Cart to found".to_string(), data : "no cart" }),

        Ok(Some(cart)) => {
            let cart_items = sqlx::query_as::<_, CartItem>("SELECT * FROM cart_items WHERE cart_id = $1")
                .bind(&cart.id)
                .fetch_all(&data.db_pool)
                .await;
            
            HttpResponse::Ok().json( ApiResponse {
                status: "Success".to_string(),
                msg: "All crat items".to_string(),
                data: cart_items.unwrap()
            })
            
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e))

    }
}

pub async fn add_to_cart(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<AddItem>
) -> impl Responder {
    
    let add_item = sqlx::query("Insert Into cart_items(cart_id, product_id, quantity) values ($1, $2, $3)")
        .bind(&payload.cart_id)
        .bind(&payload.product_id)
        .bind(&payload.quantity)
        .execute(&data.db_pool)
        .await;
    
    match add_item { 
        Ok(..) => HttpResponse::Ok().json( ApiResponse {
            status : "Success".to_string(),
            msg : "Item added to the cart successfully".to_string(),
            data: "No data"
        }),
        Err(_) => HttpResponse::Ok().json( ApiResponse {
            status : "Error".to_string(),
            msg : "Error occurred".to_string(),
            data: "No data".to_string()
        })
    }
}

pub async fn clean_cart(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
) -> impl Responder {
    let user_id = match Uuid::parse_str(&user.claims.sub) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid user ID format"),
    };

    // Fetch the user's cart
    let cart = sqlx::query_as::<_, Cart>("SELECT * FROM shopping_cart WHERE user_id = $1")
        .bind(&user_id)
        .fetch_optional(&data.db_pool)
        .await;

    match cart {
        Ok(None) => HttpResponse::Ok().json(ApiResponse {
            status: "Error".to_string(),
            msg: "No cart found".to_string(),
            data: "No data".to_string(),
        }),
        Ok(Some(cart)) => {
            // Delete all items from the cart
            let delete_result = sqlx::query("DELETE FROM cart_items WHERE cart_id = $1")
                .bind(&cart.id)
                .execute(&data.db_pool)
                .await;

            match delete_result {
                Ok(_) => HttpResponse::Ok().json(ApiResponse {
                    status: "Success".to_string(),
                    msg: "Cart cleaned successfully".to_string(),
                    data: "{}".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn remove_product_from_cart(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<AddItem>,
) -> impl Responder {
    let user_id = match Uuid::parse_str(&user.claims.sub) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid user ID format"),
    };

    // Fetch the user's cart
    let cart = sqlx::query_as::<_, Cart>("SELECT * FROM shopping_cart WHERE user_id = $1")
        .bind(&user_id)
        .fetch_optional(&data.db_pool)
        .await;

    match cart {
        Ok(None) => HttpResponse::Ok().json(ApiResponse {
            status: "Error".to_string(),
            msg: "No cart found".to_string(),
            data: "No data".to_string(),
        }),
        Ok(Some(cart)) => {
            // Check if the product exists in the cart
            let product_in_cart = sqlx::query_as::<_, CartItem>(
                "SELECT * FROM cart_items WHERE cart_id = $1 AND product_id = $2"
            )
                .bind(&cart.id)
                .bind(&payload.product_id)
                .fetch_optional(&data.db_pool)
                .await;

            match product_in_cart {
                Ok(Some(mut item)) => {
                    if payload.quantity == 0 {
                        // Remove product completely if quantity is 0
                        let remove_result = sqlx::query("DELETE FROM cart_items WHERE cart_id = $1 AND product_id = $2")
                            .bind(&cart.id)
                            .bind(&payload.product_id)
                            .execute(&data.db_pool)
                            .await;

                        match remove_result {
                            Ok(_) => HttpResponse::Ok().json(ApiResponse {
                                status: "Success".to_string(),
                                msg: "Product removed from cart".to_string(),
                                data: "{}".to_string(),
                            }),
                            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
                        }
                    } else {
                        // Update product quantity if it's greater than 0
                        let update_result = sqlx::query("UPDATE cart_items SET quantity = $1 WHERE cart_id = $2 AND product_id = $3")
                            .bind(&payload.quantity)
                            .bind(&cart.id)
                            .bind(&payload.product_id)
                            .execute(&data.db_pool)
                            .await;

                        match update_result {
                            Ok(_) => HttpResponse::Ok().json(ApiResponse {
                                status: "Success".to_string(),
                                msg: "Product quantity updated".to_string(),
                                data: "{}".to_string(),
                            }),
                            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
                        }
                    }
                }
                Ok(None) => HttpResponse::Ok().json(ApiResponse {
                    status: "Error".to_string(),
                    msg: "Product not found in cart".to_string(),
                    data: "No data".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}