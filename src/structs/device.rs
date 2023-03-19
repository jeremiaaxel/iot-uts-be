use tide::prelude::*;
use lazy_static::lazy_static;

#[derive(Serialize, Clone)]
pub enum DeviceType {
    Light,
    AirConditioner
}

impl DeviceType {
    pub fn to_string(&self) -> String {
        return match self {
            DeviceType::Light => String::from("Light"),
            DeviceType::AirConditioner => String::from("AirConditioner")
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Device {
    pub device_id: i32,
    pub device_name: String,
    pub device_type: DeviceType,
    pub device_url: String
}

impl Device {
    pub fn new(device_id: i32, device_type: DeviceType, device_name: String) -> Self {
        let device_url = String::from(format!("/{0}/{1}", device_type.to_string().to_lowercase(), device_id));
        Device {
            device_id,
            device_name,
            device_type,
            device_url
        }
    }
}

lazy_static! {
    #[derive(Clone, Copy)]
    pub static ref DEVICES: Vec<Device> = {
        let mut devices = Vec::new();
        devices.push(Device::new(1, DeviceType::Light, String::from("Light - 1")));
        devices.push(Device::new(2, DeviceType::AirConditioner, String::from("AC - 2")));
        devices.sort_by(|device1, device2| device1.device_id.cmp(&device2.device_id));

        return devices;
    };
}