use futures_util::stream::Stream;
use tokio::time::interval;
use bytes::Bytes;
use core::pin::Pin;
use core::time::Duration;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::messages::messages;

#[get("/event/{message}")]
async fn sse(_req: HttpRequest, info: web::Path<messages::Info>) -> impl Responder {

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