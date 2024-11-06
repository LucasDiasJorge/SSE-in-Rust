use actix_web::{Error, HttpRequest, HttpResponse};
use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use rdkafka::{ClientConfig, Message, Timestamp};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use async_stream::stream;
use std::pin::Pin;
use serde_json::Number;
use crate::models::event::EventPayload;
use crate::models::handshake::HandshakePayload;
use crate::services::auth::{extract_token, validate_token, AuthError};

pub struct KafkaConsumer {
    pub(crate) consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new() -> Self {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "rust-kafka-consumer")
            .set("bootstrap.servers", "localhost:9092")
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set("client.id", "rust-client")
            .set("connections.max.idle.ms", "540000000")
            .create()
            .expect("Consumer creation failed");

        KafkaConsumer { consumer }
    }
}

pub async fn handle_message(msg: BorrowedMessage<'_>) -> Result<Bytes, Error> {

    let payload = match msg.payload_view::<str>() {
        Some(Ok(payload)) => payload.to_string(),
        Some(Err(_)) => "<payload is not utf-8>".to_string(),
        None => "<payload is empty>".to_string(),
    };

    let event_payload = EventPayload {
        id: 1,
        timestamp: Number::from(msg.timestamp().to_millis().unwrap()).clone(),
        message: payload,
    };

    Ok(Bytes::from(event_payload.serialize()))
}

pub fn report_events() -> Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>> {
    let kafka_consumer = KafkaConsumer::new();
    let consumer = kafka_consumer.consumer;

    let stream = stream! {
        let handshake_id = 1;
        let handshake_payload = HandshakePayload {
            id: 1,
            event: String::from("handshake"),
        };

        let handshake_data = format!(
            "id: {}\nevent: {}\n",
            handshake_id,
            handshake_payload.event
        );

        yield Ok::<_, Error>(Bytes::from(handshake_data));

        consumer.subscribe(&["test-topic"]).expect("Can't subscribe to topics");
        let mut message_stream = consumer.stream();

        while let Some(message) = message_stream.next().await {
            match message {
                Ok(borrowed_message) => {
                    let result = handle_message(borrowed_message).await;
                    if let Ok(data) = result {
                        yield Ok::<_, Error>(data);
                    }
                },
                Err(e) => eprintln!("Kafka error: {}", e),
            }
        }
    };

    Box::pin(stream)
}

pub(crate) async fn on_connect(req: &HttpRequest) -> Result<(), HttpResponse> {

    let token = extract_token(req).map_err(|e| match e {
        AuthError::MissingToken => HttpResponse::Unauthorized().body("Missing authorization token"),
        _ => HttpResponse::InternalServerError().body("Token extraction failed"),
    })?;

    validate_token(token).await.map_err(|e| match e {
        AuthError::InvalidToken => HttpResponse::Unauthorized().body("Invalid token"),
        AuthError::ServiceUnavailable => HttpResponse::InternalServerError().body("Authentication service unreachable"),
        _ => HttpResponse::InternalServerError().body("Token validation failed"),
    })?;


    Ok(())
}