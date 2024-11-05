use actix_web::{get, web, HttpRequest, HttpResponse, Error, Responder};
use async_stream::stream;
use bytes::Bytes;
use futures_util::stream::Stream;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::interval;
use serde::Deserialize;
use serde_json::json;
use crate::services::auth::{extract_token, validate_token, AuthError};
use crate::payload::handshake::HandshakePayload;
use crate::payload::event::EventPayload;

// Função que gerencia a conexão inicial e validação de token
async fn on_connect(req: &HttpRequest, event_name: String) -> Result<(), HttpResponse> {
    // Your token validation logic (commented out for now)
    /*
    let token = extract_token(req).map_err(|e| match e {
        AuthError::MissingToken => HttpResponse::Unauthorized().body("Missing authorization token"),
        _ => HttpResponse::InternalServerError().body("Token extraction failed"),
    })?;

    validate_token(token).await.map_err(|e| match e {
        AuthError::InvalidToken => HttpResponse::Unauthorized().body("Invalid token"),
        AuthError::ServiceUnavailable => HttpResponse::InternalServerError().body("Authentication service unreachable"),
        _ => HttpResponse::InternalServerError().body("Token validation failed"),
    })?;
    */

    Ok(())
}

// Função que gera o stream de eventos SSE
fn report_events(message: String) -> Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>> {
    let stream = stream! {
        let mut interval = interval(Duration::from_secs(2));

        // Create a unique ID for the handshake
        let handshake_id = 1;

        // Send the handshake message
        let handshake_payload = HandshakePayload {
            id: 1,
            event: String::from("handshake"),
        };

        // Prepare handshake data
        let handshake_data = format!(
            "id: {}\nevent: {}\n",
            handshake_id,
            handshake_payload.event
        );

        yield Ok::<_, Error>(Bytes::from(handshake_data));

        loop {
            interval.tick().await;

            let payload_event = EventPayload {
                id: -1,
                timestamp: serde_json::Number::from(chrono::Utc::now().timestamp()),
                message: message.clone(),
            };

            // Serializar o payload para JSON
            let json_payload = serde_json::to_string(&payload_event).unwrap();

            // Formatar a mensagem SSE
            let data = format!("data: {}\n", json_payload);
            yield Ok::<_, Error>(Bytes::from(data));
        }
    };

    Box::pin(stream)
}

#[derive(Deserialize)]
struct Info {
    message: String,
}

#[get("/event/{message}")]
pub async fn sse(
    req: HttpRequest,
    info: web::Path<Info>
) -> Result<HttpResponse, Error> {
    println!("Received SSE event: {:?}", req);

    // Call on_connect to validate the connection and process the event name
    if let Err(response) = on_connect(&req, info.message.clone()).await {
        return Ok(response); // If there's an error, return the response from on_connect
    }

    // Gera o stream de eventos SSE
    let event_stream = report_events(info.message.clone());

    Ok(
        HttpResponse::Ok()
            .content_type("text/event-stream") // Define o tipo correto de conteúdo SSE
            .streaming(event_stream)
    )
}
