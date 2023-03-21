use async_std::sync::Mutex;
use tide::prelude::*;
use chrono::{DateTime, Local};

use super::device::DeviceType;

use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceStatus {
    pub device_type: DeviceType,
    pub device_id: i32,
    pub status: String, // on / off
    pub temperature: i32, // 16 - 30
    pub is_timer_on: bool,
    pub is_timer_off: bool,
    pub timer_on: String, // 00:00 - 23:59
    pub timer_off: String, // 00:00 - 23:59
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceStatusLU {
    pub device_type: DeviceType,
    pub device_id: i32,
    pub status: String, // on / off
    pub temperature: i32, // 16 - 30
    pub is_timer_on: bool,
    pub is_timer_off: bool,
    pub timer_on: String, // 00:00 - 23:59
    pub timer_off: String, // 00:00 - 23:59
    pub last_updated: DateTime<Local>
}

#[allow(dead_code)]
impl DeviceStatus {
    pub fn new(device_id: i32, device_type: DeviceType) -> Self {
        let status = "Off".to_string();
        let temperature = 16;
        let is_timer_on = false;
        let is_timer_off = false;
        let timer_on = "00:00".to_string();
        let timer_off = "00:00".to_string();

        DeviceStatus {
            device_type,
            device_id,
            status,
            temperature,
            is_timer_on,
            is_timer_off,
            timer_on,
            timer_off,
        }
    }

    pub fn clone_from_other(&mut self, device_status: DeviceStatus) {
        self.status = device_status.status;
        self.temperature = device_status.temperature;
        self.is_timer_on = device_status.is_timer_on;
        self.is_timer_off = device_status.is_timer_off;
        self.timer_on = device_status.timer_on;
        self.timer_off = device_status.timer_off;
    }
}

#[allow(dead_code)]
impl DeviceStatusLU {
    pub fn new(device_id: i32, device_type: DeviceType) -> Self {
        let status = "Off".to_string();
        let temperature = 16;
        let is_timer_on = false;
        let is_timer_off = false;
        let timer_on = "00:00".to_string();
        let timer_off = "00:00".to_string();
        let last_updated = Local::now();

        DeviceStatusLU {
            device_type,
            device_id,
            status,
            temperature,
            is_timer_on,
            is_timer_off,
            timer_on,
            timer_off,
            last_updated
        }
    }

    pub fn clone_from_other(&mut self, device_status: DeviceStatusLU) {
        self.status = device_status.status;
        self.temperature = device_status.temperature;
        self.is_timer_on = device_status.is_timer_on;
        self.is_timer_off = device_status.is_timer_off;
        self.timer_on = device_status.timer_on;
        self.timer_off = device_status.timer_off;
        self.last_updated = device_status.last_updated;
    }

    pub fn clone_from_no_lu(&mut self, device_status: DeviceStatus) {
        self.status = device_status.status;
        self.temperature = device_status.temperature;
        self.is_timer_on = device_status.is_timer_on;
        self.is_timer_off = device_status.is_timer_off;
        self.timer_on = device_status.timer_on;
        self.timer_off = device_status.timer_off;
        self.last_updated = Local::now();
    }
}



lazy_static! {
    pub static ref DEVICES_STATUS: Mutex<Vec<DeviceStatusLU>> = {
        let mut devices_status = Vec::new();
        devices_status.push(DeviceStatusLU::new(1, DeviceType::Light));
        devices_status.push(DeviceStatusLU::new(2, DeviceType::AirConditioner));
        Mutex::new(devices_status)
    };
}