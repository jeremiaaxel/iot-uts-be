use tide::prelude::*;

#[derive(Serialize)]
pub struct Lamp {
    pub device_id: i32,
    pub device_name: String,
}