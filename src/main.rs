use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;
mod models;
mod handlers;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let db = db::DB::init()
        .await
        .expect("Failed to connect to database");

    println!("Server running at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // db is now Arc<DB>, which supports clone
            .route("/users", web::post().to(handlers::user_handlers::create_user))
            .route("/users", web::get().to(handlers::user_handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::user_handlers::get_user))
            .route("/users/{id}", web::put().to(handlers::user_handlers::update_user))
            .route("/users/{id}", web::delete().to(handlers::user_handlers::delete_user))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}