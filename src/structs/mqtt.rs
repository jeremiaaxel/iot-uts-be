use mqtt::DisconnectOptions;
use paho_mqtt as mqtt;
use tide::log;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Mqtt {
    client_name: String,
    uri: String,
    port: String,
    topic: String,
    client: mqtt::Client,
    qos: i32,
}

#[allow(dead_code)]
impl Mqtt {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let client_name = std::env::var("MQTT_CLIENT_NAME").expect("MQTT_CLIENT_NAME not set");
        let uri = std::env::var("MQTT_URI").expect("MQTT_URI not set");
        let port = std::env::var("MQTT_PORT").expect("MQTT_PORT not set");

        let led_pin = 2;
        let topic = String::from(format!("{client_name}/server_led_state_{led_pin}"));
        let qos: i32 = 0;

        let client_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(uri.clone())
            .finalize();

        let client = mqtt::Client::new(client_opts).expect("Error creating client");

        return Mqtt { client_name, uri, port, topic, client, qos };
    }

    pub fn connect(&self) {
        log::info!("{}", String::from(format!("Connecting to MQTT broker at... {0}:{1}", self.uri, self.port)));
        let options = mqtt::ConnectOptionsBuilder::new()
            .clean_session(true)
            .finalize();

        self.client
            .connect(options)
            .expect("Error connecting to MQTT broker");
    }

    pub fn disconnect(&self) {
        log::info!("Disconnecting to MQTT broker...");
        self.client
            .disconnect(DisconnectOptions::default())
            .expect("Error disconnecting from MQTT broker");
    }

    pub fn publish(&self, msg: String) {
        log::info!("Publishing message to MQTT broker...");
        self.client.publish(mqtt::Message::new(self.topic.clone(), msg, self.qos))
            .expect("Error publishing message to MQTT broker");
    }

    pub fn subscribe(&self) {
        log::info!("Subscribing to MQTT broker...");
        todo!("Subscribe to MQTT broker")
    }

    pub fn unsubscribe(&self) {
        log::info!("Unsubscribing from MQTT broker...");
        todo!("Unsubscribe from MQTT broker")
    }

}
