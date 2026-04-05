use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod db;
mod handler;
mod models;
mod schema;
mod user_handler;
mod auth;
mod fixtures;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url);
    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/orders", web::get().to(handler::get_orders))
            .route("/orders", web::post().to(handler::create_order))
            .route("/register", web::post().to(user_handler::register_user))
            .route("/login", web::post().to(user_handler::login_user))
    })

    .bind("127.0.0.1:8080")?
    .run()
    .await
    
}