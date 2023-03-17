use crate::TEMPLATES;
use super::mqtt::Mqtt;

#[derive(Clone)]
pub struct State {
    pub tera: TEMPLATES,
    pub mqtt: Mqtt,
}