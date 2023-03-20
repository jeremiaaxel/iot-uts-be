use tide::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceStatus {
    pub status: String, // on / off
    pub temperature: i32, // 16 - 30
    pub is_timer_on: bool,
    pub is_timer_off: bool,
    pub timer_on: String, // 00:00 - 23:59
    pub timer_off: String, // 00:00 - 23:59
}
