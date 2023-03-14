use mqtt::DisconnectOptions;
use paho_mqtt as mqtt;
use tide::log;

#[allow(dead_code)]
pub(crate) struct Mqtt {
    client_name: String,
    uri: String,
    port: u16,
    topic: String,
    client: mqtt::Client,
    qos: i32,
}

#[allow(dead_code)]
impl Mqtt {
    pub fn new() -> Self {
        let client_name = String::from("13519188");
        let uri = String::from("192.168.211.59");
        let port = 1883;
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
        log::info!("Connecting to MQTT broker...");
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
