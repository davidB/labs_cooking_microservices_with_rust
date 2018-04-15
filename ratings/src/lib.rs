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

pub mod example;
mod config;

lazy_static! {
    static ref CONFIG: config::Config = config::Config::new();
}

pub fn start() {
    let addr = format!("{}:{}", CONFIG.host, CONFIG.port);
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, example::router())
}
