use tide::Request;
use tide_tera::{TideTeraExt, context};

use crate::structs;
use structs::state::State;

pub async fn index_view(mut _req: Request<State>) -> tide::Result {
    let tera = _req.state().tera.clone();
    let title = String::from("Hello, there!");
    let lamps_index_url = String::from("/lamps");
    
    tera.render_response("index.html", &context! { 
        "title" => title,
        "lamps_index_url" => lamps_index_url
    })
}