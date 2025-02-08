mod routes;
mod models;

use actix_web::{web, App, HttpServer};
use routes::create_user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/create_user", web::post().to(create_user))  // Register the route
    })
    .bind("127.0.0.1:8080")?  // Bind the server to localhost on port 8080
    .run()
    .await
}
