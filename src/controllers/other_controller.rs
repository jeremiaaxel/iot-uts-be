use tide::Request;
use tide_tera::{TideTeraExt, context};

use crate::structs;
use structs::state::State;

pub async fn hello_world(mut _req: Request<State>) -> tide::Result {
    Ok(format!("Hello, world!").into())
}

pub async fn index_view(mut _req: Request<State>) -> tide::Result {
    let tera = _req.state().tera.clone();
    tera.render_response("index.html", &context! { 
        "title" => String::from("Hello, world!")
    })
}