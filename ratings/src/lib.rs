extern crate gotham;
#[macro_use]
extern crate gotham_derive;

extern crate futures;
extern crate hyper;
extern crate mime;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

extern crate time;

#[macro_use]
extern crate log;

use gotham::router::Router;
use gotham::router::builder::*;

mod health;
mod config;
mod ratings;
mod db;

lazy_static! {
    static ref CONFIG: config::Config = config::Config::new();
}

pub fn router() -> Router {
    build_simple_router(|route| {
        route.get("/health").to(health::healthcheck);

        route
            .get("/ratings/:product_id")
            .with_path_extractor::<ratings::ProductIdPathExtractor>()
            .to(ratings::ratings);

        route
            .post("/ratings/:product_id")
            .with_path_extractor::<ratings::ProductIdPathExtractor>()
            .to(ratings::save_rating);
    })
}

pub fn start() {
    let addr = format!("{}:{}", CONFIG.host, CONFIG.port);
    info!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
