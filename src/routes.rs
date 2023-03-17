use crate::controllers::other_controller;
use crate::controllers::lamp_controller;
use crate::controllers::home_controller;

// use csrf::{AesGcmCsrfProtection, CsrfProtection};
// use data_encoding::BASE64;

use crate::State;

pub fn register_routes(app: &mut tide::Server<State>) {
    // Other Controller
    app.at("/").get(home_controller::index_view);
    app.at("/hello_world").get(other_controller::hello_world);

    // Lamp Controller
    app.at("/lamps").get(lamp_controller::index);
    app.at("/lamps/:device_id").get(lamp_controller::detail);
    app.at("/lamps/:device_id/publish").post(lamp_controller::mqtt_publish);
}
