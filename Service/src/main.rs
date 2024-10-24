use core::pin::Pin;
use core::time::Duration;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures_util::stream::Stream;
use tokio::time::interval;
use bytes::Bytes;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    message: String,
}

#[get("/event/{message}")]
async fn sse(_req: HttpRequest, info: web::Path<Info>) -> impl Responder {

    let stream = async_stream::stream! {
        let mut interval = interval(Duration::from_secs(2));

        loop {
            interval.tick().await;
            let data = format!("data: {{ \"message\": \"Event {}\" }}\n\n", info.message);
            yield Ok::<_, actix_web::Error>(Bytes::from(data)); // Temporariamente retorna "data" sem encerrar a execução do bloco assíncrono.
        }
    };

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(Box::pin(stream) as Pin<Box<dyn Stream<Item = Result<Bytes, actix_web::Error>>>>)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(sse)
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
