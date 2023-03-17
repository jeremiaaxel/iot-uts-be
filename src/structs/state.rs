use tera::Tera;
use super::mqtt::Mqtt;

#[derive(Clone)]
pub struct State {
    pub tera: Tera,
    pub mqtt: Mqtt,
}