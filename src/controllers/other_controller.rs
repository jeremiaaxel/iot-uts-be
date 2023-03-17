use tide::Request;

use crate::structs;
use structs::state::State;

pub async fn hello_world(mut _req: Request<State>) -> tide::Result {
    Ok(format!("Hello, world!").into())
}
