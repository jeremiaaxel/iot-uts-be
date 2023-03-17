use tide::prelude::*;

#[derive(Deserialize)]
pub struct LampStatus {
    pub status: String,
}