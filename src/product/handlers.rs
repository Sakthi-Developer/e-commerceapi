use crate::AppState;
use crate::auth::models::User;
use crate::product::models::Product;
use crate::routes::models::ApiResponse;
use actix_web::{HttpResponse, Responder, web};
use chrono::Month::April;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub async fn get_all_products(data: web::Data<AppState>) -> impl Responder {
    let products = sqlx::query_as::<_, Product>("select * from products")
        .fetch_all(&data.db_pool)
        .await;

    match products {
        Ok(products) => {
            return HttpResponse::Ok().json(ApiResponse {
                status: "Success".to_string(),
                msg: "All products".to_string(),
                data: products,
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error: {}", e));
        }
    }
}

pub async fn get_product_by_id(data: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    let product = sqlx::query_as::<_, Product>("select * from products where id = $1")
        .bind(*id)
        .fetch_one(&data.db_pool)
        .await;

    match product {
        Ok(product) => HttpResponse::Ok().json(ApiResponse {
            status: "Success".to_string(),
            msg: "Product Details".to_string(),
            data: product,
        }),
        Err(_) => HttpResponse::Ok().json(ApiResponse {
            status: "Error".to_string(),
            msg: "Error while fetching data for the Id".to_string(),
            data: "No Data".to_string(),
        }),
    }
}

pub async fn search_product(data: web::Data<AppState>, query: web::Query<String>) -> impl Responder {
    let search_term = &query.into_inner();

    // Search the products by name or any other attributes, you can modify this query based on your needs
    let products = sqlx::query_as::<_, Product>(
        "SELECT * FROM products WHERE LOWER(name) LIKE LOWER($1) OR LOWER(description) LIKE LOWER($1)"
    )
        .bind(format!("%{}%", search_term))  // Bind the search term with wildcards for a partial match
        .fetch_all(&data.db_pool)
        .await;

    match products {
        Ok(products) => {
            if products.is_empty() {
                return HttpResponse::Ok().json(ApiResponse {
                    status: "Error".to_string(),
                    msg: "No products found".to_string(),
                    data: "No Data".to_string(),
                });
            }
            return HttpResponse::Ok().json(ApiResponse {
                status: "Success".to_string(),
                msg: "Search results".to_string(),
                data: products,
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error: {}", e));
        }
    }
}