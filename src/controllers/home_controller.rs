use tide::{Request};
use tide_tera::{TideTeraExt, context};

use crate::structs;
use structs::state::State;
use structs::device::{Device, DEVICES};

pub async fn index(mut _req: Request<State>) -> tide::Result {
    let tera = _req.state().tera.clone();
    let title = String::from("Devices");
    let devices: Vec<Device> = DEVICES.to_vec();

    tera.render_response("index.html", &context! { 
        "title" => title,
        "devices" => devices
    })
}