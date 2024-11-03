use actix_web::{App, HttpServer};
use serde::Deserialize;

mod sse;
mod messages;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(sse::controller::sse)
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
