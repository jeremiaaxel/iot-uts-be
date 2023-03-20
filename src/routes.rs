use crate::controllers::other_controller;
use crate::controllers::device_controller;
use crate::controllers::home_controller;

// use csrf::{AesGcmCsrfProtection, CsrfProtection};
// use data_encoding::BASE64;

use crate::State;

pub fn register_routes(app: &mut tide::Server<State>) {
    // Other Controller
    app.at("/public").serve_dir("src/public").expect("Error serving static files");
    app.at("/").get(home_controller::index);
    app.at("/hello_world").get(other_controller::hello_world);

    // Device Controller
    app.at("/:device_type/:device_id").get(device_controller::detail);
    app.at("/:device_type/:device_id/publish").post(device_controller::mqtt_publish);
}
