use rdkafka::producer::{FutureProducer};
use rdkafka::config::ClientConfig;

pub mod conf;

pub struct KafkaProducer {}

impl KafkaProducer {
    pub fn init() -> FutureProducer {        
        let brokers = conf::Props::init().kafka_brokers;
        let mut kafka_config = ClientConfig::new();
        let producer = kafka_config
            .set("message.timeout.ms", "2000")
            .set("bootstrap.servers", brokers)
            .create::<FutureProducer>().expect("An error occured while initializing producer.");

        producer
    }
}