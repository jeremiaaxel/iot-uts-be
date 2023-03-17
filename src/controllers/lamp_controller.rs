use tide::Request;
use tide_tera::{TideTeraExt, context};
use tide::log;
use structs::lamp_status::LampStatus;
// use csrf::{AesGcmCsrfProtection, CsrfProtection};
// use data_encoding::BASE64;

use crate::structs;
use structs::state::State;

pub async fn lamp_view(mut _req: Request<State>) -> tide::Result {
    let tera = _req.state().tera.clone();
    tera.render_response("lamp.html", &context! { 
        "title" => String::from("Lamp!"),
        "url" => "/lamp/click"
    })
}

pub async fn lamp_post(mut _req: Request<State>) -> tide::Result {
    let body: LampStatus = _req.body_form().await.unwrap();
    let lamp_status = body.status.trim();

    log::info!("Lamp status: {}", lamp_status);
    _req.state().mqtt.publish(String::from(lamp_status));
    Ok(format!("bar").into())
}