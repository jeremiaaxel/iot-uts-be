use tide::prelude::*;

#[derive(Deserialize, Debug)]
pub struct DeviceBody {
    pub status: String, // on / off
    pub temperature: Option<i32>, // 16 - 30
    // pub fan_speed: i32, // 1 - 5
    // pub mode: String, // cool / dry / fan / auto
}
