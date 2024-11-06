use actix_web::{get, web, HttpRequest, HttpResponse, Error, Responder};
use crate::services::connection_handler::{report_events, on_connect};
use crate::models::info::Info;

#[get("/event/{message}")]
pub async fn sse(
    req: HttpRequest,
    info: web::Path<Info>
) -> Result<HttpResponse, Error> {

    println!("Received SSE event: {:?}", req);

    //if let Err(response) = on_connect(&req).await {
    //    return Ok(response);
    //}

    let event_stream = report_events();

    Ok(
        HttpResponse::Ok()
            .content_type("text/event-stream")
            .streaming(event_stream)
    )
}
