use tide::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceStatus {
    pub status: String, // on / off
    pub temperature: Option<i32>, // 16 - 30
    // pub fan_speed: i32, // 1 - 5
    // pub mode: String, // cool / dry / fan / auto
}

impl DeviceStatus {
    pub fn new(status: String, temperature: Option<i32>) -> Self {
        DeviceStatus {
            status: status,
            temperature: temperature,
        }
    }

    pub fn to_string(&self) -> String {
        let mut result_string = String::from(format!("status: {}", self.status));
        if let Some(temperature) = self.temperature {
            result_string = String::from(format!("{}, temperature: {}", result_string, temperature));
        }
        return result_string;
    }
}