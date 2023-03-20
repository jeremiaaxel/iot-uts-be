mod routes;
use std::collections::VecDeque;
use routes::register_routes;

mod controllers;

use structs::device::DeviceType;
use tera::Tera;
use tide::log;

mod structs;
use structs::state::State;
use structs::mqtt::Mqtt;

use lazy_static::lazy_static;

use crate::structs::device_status::DeviceStatus;

lazy_static! {
    #[derive(Clone, Copy)]
    pub static ref TEMPLATES: Tera = {
        // Tera
        let mut tera = Tera::new("src/views/**/*.html")
                            .expect("Error parsing templates directory");
        tera.autoescape_on(vec!["html"]);
        return tera;
    };
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().ok();
    log::start();

    // MQTT
    let mqtt = Mqtt::new();
    mqtt.connect().await;
    
    let tera = TEMPLATES;

    let state = State { 
        tera:tera, 
        mqtt:mqtt.clone(),
    };

    // App
    let mut app = tide::with_state(state);
    register_routes(&mut app);
    let listen_to = std::env::var("WEB_URI").unwrap() + ":" + &std::env::var("WEB_PORT").unwrap();
    app.listen(listen_to).await?;
    
    // App done
    // MQTT
    mqtt.disconnect().await;

    Ok(())
}