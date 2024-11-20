use actix_web::{get, web, HttpRequest, HttpResponse, Error};
use crate::services::connection_handler::{on_connect, route_to_event};
use crate::models::info::Info;

#[get("/event/{event_type}")]
pub async fn sse(
    req: HttpRequest,
    info: web::Path<Info>
) -> Result<HttpResponse, Error> {
    println!("Received SSE event: {:?}", req);

    if let Err(response) = on_connect(&req).await {
        return Ok(response);
    }

    let event_type = info.into_inner().event_type;

    let event_stream = route_to_event(event_type);

    Ok(
        HttpResponse::Ok()
            .content_type("text/event-stream")
            .streaming(event_stream)
    )
}
