pub mod models;

use actix_web::web;
use actix_web::web::{route, to};
use crate::auth::handlers as auth_handlers;
use crate::product::handlers as product_handlers;
use crate::cart::handlers as cart_handlers;
use crate::order::handlers as order_handlers;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/product/all", web::get().to(product_handlers::get_all_products))   // Get all products
        .route("/product/{id}", web::get().to(product_handlers::get_product_by_id)) // Get product by ID
        .route("/search", web::get().to(product_handlers::search_product))          // Change to GET for search
        .route("/create_cart", web::get().to(cart_handlers::create_cart))           // Create a new cart
        .route("/addToCart", web::post().to(cart_handlers::add_to_cart))            // Add product to cart
        .route("/myCart", web::get().to(cart_handlers::get_cart_items))             // Get cart items
        .route("/flushCart", web::get().to(cart_handlers::clean_cart))              // Clean cart
        .route("/removeItem-cart", web::get().to(cart_handlers::remove_product_from_cart))// Fixed typo
        .route("/checkout", web::get().to(order_handlers::create_checkout))         // Checkout route
        .route("/signUp", web::post().to(auth_handlers::sign_up))                   // Sign up route
        .route("/logIn", web::post().to(auth_handlers::log_in));                                     // Log in route
}
