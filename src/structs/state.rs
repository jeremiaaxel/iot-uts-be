use std::collections::VecDeque;

use crate::TEMPLATES;
use super::{mqtt::Mqtt, device_status::DeviceStatus};

#[derive(Clone)]
pub struct State {
    pub tera: TEMPLATES,
    pub mqtt: Mqtt,
}