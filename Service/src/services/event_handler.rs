use std::pin::Pin;
use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::Error;
use async_stream::stream;
use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use serde_json::Number;
use crate::models::event::{Content, EventPayload};
use crate::models::greeating::GreetingsPayload;

pub struct KafkaConsumer {
    pub(crate) consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new() -> Self {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "rust-kafka-consumer")
            .set("bootstrap.servers", "localhost:9092")
            .set("enable.partition.eof", "false")
            .set("enable.auto.commit", "true")
            .set("client.id", "rust-client")
            .set("connections.max.idle.ms", "540000000")
            .set("auto.offset.reset", "latest")
            .create()
            .expect("Consumer creation failed");

        KafkaConsumer { consumer }
    }
}

pub async fn handle_message(msg: BorrowedMessage<'_>) -> Result<Option<Bytes>, Error> {

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64;

    let payload = match msg.payload_view::<str>() {
        Some(Ok(payload)) => payload.to_string(),
        Some(Err(_)) => "<payload is not utf-8>".to_string(),
        None => "<payload is empty>".to_string(),
    };

    let content: Content = match serde_json::from_str(&payload) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to parse content from payload: {}", payload);
            Content {
                epc: "<missing epc>".to_string(),
                timestamp: Number::from(0),
                newLocation: "".to_string(),
                lastLocation: "".to_string(),
                sku: "".to_string(),
            }
        }
    };

    let message_timestamp = msg.timestamp().to_millis().unwrap_or(0);

    const ONE_MINUTE_IN_MILLIS: i64 = 60 * 1000;
    if message_timestamp < current_time - ONE_MINUTE_IN_MILLIS {
        return Ok(None);
    }

    let event_payload = EventPayload {
        id: 1,
        timestamp: Number::from(message_timestamp),
        isRead: false,
        message: "OK".to_string(),
        event: "".to_string(),
        title: "Real Time (Reading)".to_string(),
        content,
        company: "test-topic".to_string(),
    };

    let data = format!("data: {}", event_payload.serialize());

    Ok(Some(Bytes::from(data)))
}

pub fn report_events() -> Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>> {
    let kafka_consumer = KafkaConsumer::new();
    let consumer = kafka_consumer.consumer;

    let stream = stream! {
        let handshake_id = 1;

        let handshake_payload = GreetingsPayload {
            id: 1,
            event: String::from("handshake"),
        };

        let handshake_data = format!(
            "id: {}\nevent: {}\n",
            handshake_id,
            handshake_payload.event
        );

        yield Ok::<_, Error>(Bytes::from(handshake_data));

        consumer.subscribe(&["00162515910e"]).expect("Can't subscribe to topics");
        let mut message_stream = consumer.stream();

        while let Some(message) = message_stream.next().await {
            match message {
                Ok(borrowed_message) => {
                    if let Ok(Some(data)) = handle_message(borrowed_message).await {
                        yield Ok::<_, Error>(data);
                    }
                },
                Err(e) => eprintln!("Kafka error: {}", e),
            }
        }
    };

    Box::pin(stream)
}

pub fn report_pong() -> Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>> {
    let kafka_consumer = KafkaConsumer::new();
    let consumer = kafka_consumer.consumer;

    let stream = stream! {
        let handshake_id = 1;

        let greeting_payload = GreetingsPayload {
            id: 1,
            event: String::from("pong"),
        };

        let greeting_data = format!(
            "id: {}\nevent: {}\n",
            handshake_id,
            greeting_payload.event
        );

        yield Ok::<_, Error>(Bytes::from(greeting_data));

    };

    Box::pin(stream)
}

pub fn default() -> Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>> {
    let kafka_consumer = KafkaConsumer::new();
    let _consumer = kafka_consumer.consumer;

    let stream = stream! {
        let handshake_id = 1;

        let greeting_payload = GreetingsPayload {
            id: 1,
            event: String::from("default_event"),
        };

        let greeting_data = format!(
            "id: {}\nevent: {}\n",
            handshake_id,
            greeting_payload.event
        );

        yield Ok::<_, Error>(Bytes::from(greeting_data));

    };

    Box::pin(stream)
}