use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures_util::stream::Stream;
use std::{pin::Pin, time::Duration};
use tokio::time::interval;
use bytes::Bytes;

async fn sse(_req: HttpRequest) -> impl Responder {
    let stream = async_stream::stream! {
        let mut interval = interval(Duration::from_secs(2));
        let mut counter = 0;

        loop {
            interval.tick().await;
            counter += 1;
            let data = format!("data: {{ \"message\": \"Event {}\" }}\n\n", counter);
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
            .route("/events", web::get().to(sse))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
