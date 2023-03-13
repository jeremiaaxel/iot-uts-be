mod routes;
use routes::register_routes;

mod controllers;

use tera::Tera;
use tide::log;

mod structs;
use structs::state::State;

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();

    let mut tera = Tera::new("src/views/*").expect("Error parsing templates directory");
    tera.autoescape_on(vec!["html"]);

    let state = State { tera };
    let mut app = tide::with_state(state);
    register_routes(&mut app);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}