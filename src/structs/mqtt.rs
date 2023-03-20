use mqtt::{DisconnectOptions, Message};
use paho_mqtt as mqtt;
use tide::log;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Mqtt {
    client_name: String,
    uri: String,
    port: String,
    client: mqtt::AsyncClient,
    qos: i32,
}

#[allow(dead_code)]
impl Mqtt {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let client_name = std::env::var("MQTT_CLIENT_NAME").expect("MQTT_CLIENT_NAME not set");
        let uri = std::env::var("MQTT_URI").expect("MQTT_URI not set");
        let port = std::env::var("MQTT_PORT").expect("MQTT_PORT not set");

        let qos: i32 = 0;

        let client_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(uri.clone())
            .finalize();

        let client = mqtt::AsyncClient::new(client_opts).expect("Error creating client");

        return Mqtt { client_name, uri, port, client, qos };
    }

    pub async fn connect(&self) {
        log::info!("{}", String::from(format!("Connecting to MQTT broker at... {0}:{1}", self.uri, self.port)));
        let options = mqtt::ConnectOptionsBuilder::new()
            .clean_session(true)
            .finalize();

        self.client
            .connect(options).await
            .expect("Error connecting to MQTT broker");
    }

    pub async fn disconnect(&self) {
        log::info!("Disconnecting to MQTT broker...");
        self.client
            .disconnect(DisconnectOptions::default()).await
            .expect("Error disconnecting from MQTT broker");
    }

    pub async fn publish(&self, topic: String, msg: String) {
        log::info!("Publishing message to MQTT broker...");
        let full_topic = String::from(format!("{0}/{1}", self.client_name, topic));
        self.client.publish(mqtt::Message::new(full_topic, msg, self.qos)).await
            .expect("Error publishing message to MQTT broker");
    }

    pub async fn subscribe(&self, topics: Vec<String>) {
        log::info!("Subscribing to MQTT broker...");
        let mut full_topics: Vec<String> = Vec::new();
        let mut qos = Vec::new();
        for topic in topics {
            full_topics.push(String::from(format!("{0}/{1}", self.client_name, topic)));
            qos.push(self.qos);
        }
        self.client.subscribe_many(&full_topics, &qos).await
            .expect("Error subscribing to MQTT broker");
    }

    pub async fn start_consuming(&self) -> mqtt::Receiver<Option<Message>> {
        log::info!("Starting to consume messages from MQTT broker...");
        return self.client.start_consuming();
    }

    pub fn unsubscribe(&self) {
        log::info!("Unsubscribing from MQTT broker...");
        todo!("Unsubscribe from MQTT broker")
    }

    // pub async fn set_message_callback(&self, callback: fn(Message)) {
    //     log::info!("Setting callback for MQTT broker...");
    //     self.client.set_message_callback(callback);
    // }

}
