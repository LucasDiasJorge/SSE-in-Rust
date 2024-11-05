use actix_web::{App, HttpServer};

mod sse;
mod payload;
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
