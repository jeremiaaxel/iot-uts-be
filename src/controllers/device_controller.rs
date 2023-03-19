use tide::{Request, Response, Body};
use tide_tera::{TideTeraExt, context};
use tide::log;

use crate::structs::device_status::{DeviceStatus};
use crate::structs;
use structs::state::State;
use structs::device::{Device, DEVICES};


pub async fn detail(mut _req: Request<State>) -> tide::Result {
    let device_type: String = _req.param("device_type")?.parse().unwrap();
    let device_id: i32 = _req.param("device_id")?.parse().unwrap();
    let device: Device = DEVICES.to_vec().into_iter().find(|device| device.device_type.to_string().to_lowercase() == device_type && device.device_id == device_id).unwrap();

    let publish_url = String::from(format!("{0}/publish", device.device_url.clone()));
    
    let tera = _req.state().tera.clone();
    tera.render_response("device/detail.html", &context! { 
        "title" => device.device_name,
        "device" => device,
        "url" => publish_url,
    })
}

pub async fn mqtt_publish(mut _req: Request<State>) -> tide::Result {
    let device_type: String = _req.param("device_type")?.parse().unwrap();
    let device_id: i32 = _req.param("device_id")?.parse().unwrap();
    let device: Device = DEVICES.to_vec().into_iter().find(|device| device.device_type.to_string().to_lowercase() == device_type && device.device_id == device_id).unwrap();

    let mut device_status: DeviceStatus = _req.body_form().await?;
    log::info!("{:?}", device_status);
    device_status.status = device_status.status.trim().to_string();
    
    let topic = String::from(format!("server_{0}_{1}", device.device_type.to_string(), device.device_id));
    
    log::info!("Device {}|{} status: {}", device.device_type.to_string(), device.device_id, device_status.status);
    _req.state().mqtt.publish(topic, device_status.to_string());
    
    let res = Response::new(200);
    Ok(res)
}