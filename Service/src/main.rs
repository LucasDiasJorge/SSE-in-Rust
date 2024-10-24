use actix_web::{App, HttpServer};
use serde::Deserialize;

mod controllers;
mod messages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(controllers::controller::sse)
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
