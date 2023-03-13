use crate::controllers::other_controller;
use crate::controllers::lamp_controller;

// use csrf::{AesGcmCsrfProtection, CsrfProtection};
// use data_encoding::BASE64;

use crate::State;

pub fn register_routes(app: &mut tide::Server<State>) {
    // Other Controller
    app.at("/hello_world").get(other_controller::hello_world);
    app.at("/").get(other_controller::index_view);

    // Lamp Controller
    app.at("/lamp").get(lamp_controller::lamp_view);
    app.at("/lamp/click").post(lamp_controller::lamp_post);
}
