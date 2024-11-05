use futures_util::StreamExt;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new() -> Self {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "rust-kafka-consumer")
            .set("bootstrap.servers", "localhost:9092")
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .create()
            .expect("Consumer creation failed");

        KafkaConsumer { consumer }
    }
}
pub async fn handle_message(msg: BorrowedMessage<'_>) {
    let payload = match msg.payload_view::<str>() {
        Some(Ok(payload)) => payload,
        Some(Err(_)) => "<payload is not utf-8>",
        None => "<payload is empty>",
    };

    let topic = msg.topic();
    let partition = msg.partition();
    let offset = msg.offset();

    println!(
        "Received: topic: {}, partition: {}, offset: {}, payload: {}",
        topic, partition, offset, payload
    );
}

pub async fn consumer_loop(consumer: StreamConsumer) {

    // Subscribe to all topics using a regex pattern
    consumer
        .subscribe(&["^.*"])
        .expect("Can't subscribe to specified topics");

    println!("Waiting for messages from all topics...");

    let mut message_stream = consumer.stream();

    while let Some(message) = message_stream.next().await {
        match message {
            Ok(borrowed_message) => handle_message(borrowed_message).await,
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }
}