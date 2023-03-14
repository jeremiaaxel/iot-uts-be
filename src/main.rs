mod routes;
use routes::register_routes;

mod controllers;

use tera::Tera;
use tide::log;

mod structs;
use structs::state::State;
use structs::mqtt::Mqtt;

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();

    // MQTT
    let mqtt = Mqtt::new();
    mqtt.connect();
    for _ in 0..5 {
        mqtt.publish(String::from("Hello, world!"));
    }

    // Tera
    let mut tera = Tera::new("src/views/*").expect("Error parsing templates directory");
    tera.autoescape_on(vec!["html"]);

    // App
    let state = State { tera };
    let mut app = tide::with_state(state);
    register_routes(&mut app);
    app.listen("127.0.0.1:8080").await?;
    
    // App done
    // MQTT
    mqtt.disconnect();

    Ok(())
}