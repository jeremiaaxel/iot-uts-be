use tide::Request;
use tide_tera::{TideTeraExt, context};
use tide::log;
use structs::lamp_status::LampStatus;
// use csrf::{AesGcmCsrfProtection, CsrfProtection};
// use data_encoding::BASE64;

use crate::structs;
use structs::state::State;
use structs::lamp::Lamp;

pub async fn index(mut _req: Request<State>) -> tide::Result {
    let tera = _req.state().tera.clone();
    let mut devices = Vec::new();
    let title = String::from("Lamps");
    devices.push(Lamp { device_id: 2, device_name: String::from("LED - 2") });

    tera.render_response("lamps/index.html", &context! { 
        "title" => title,
        "device_url" => String::from("/lamps"),
        "devices" => devices
    })
}

pub async fn detail(mut _req: Request<State>) -> tide::Result {
    let tera = _req.state().tera.clone();
    let device_id = _req.param("device_id")?;
    let device_name = String::from(format!("LED - {device_id}"));
    let lamp_click_url = String::from(format!("/lamps/{device_id}/publish"));

    tera.render_response("lamps/detail.html", &context! { 
        "title" => device_name,
        "device_name" => device_name,
        "url" => lamp_click_url,
        "device_id" => device_id
    })
}

pub async fn mqtt_publish(mut _req: Request<State>) -> tide::Result {
    let body: LampStatus = _req.body_form().await.unwrap();
    let device_id = _req.param("device_id")?;
    let lamp_status = body.status.trim();
    let new_lamp_status = if lamp_status == "Off" { "On" } else { "Off" };

    log::info!("Lamp {} status: {}", device_id, lamp_status);
    _req.state().mqtt.publish(String::from(lamp_status));
    Ok(format!("{new_lamp_status}").into())
}