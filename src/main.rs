use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use std::env;
use dotenv::dotenv;
use env_logger::{init, Env};
use sqlx::PgPool;
use stripe::Client;

mod db;
mod routes;
mod auth;
mod product;
mod cart;
mod order;
mod models;

use db::pool::init_db_pool;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
    stripe_client: Client,
    stripe_secret: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let db_pool = init_db_pool(&database_url).await.expect("Failed to create pool");

    let stripe_secret = env::var("STRIPE_SECRET").unwrap();
    let stripe_client = stripe::Client::new(&stripe_secret);


    let app_state = AppState {
        db_pool,
        stripe_client,
        stripe_secret,
    };

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
        .app_data(web::Data::new(app_state.clone())) 
        .wrap(Logger::default())
        .wrap(cors)
        .configure(routes::config) 
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}