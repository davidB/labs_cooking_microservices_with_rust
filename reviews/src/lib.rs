extern crate actix_web;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate time;

#[macro_use]
extern crate lazy_static;

use actix_web::middleware::Logger;
use actix_web::{http, server, App};

mod config;
mod health;
mod example;

lazy_static! {
    static ref CONFIG: config::Config = config::Config::new();
}

pub fn run(addr: &str) {
    server::new(move || {
        App::new()
            .middleware(Logger::default())
            .resource("/health", |r| {
                r.method(http::Method::GET).f(health::healthcheck)
            })
            .resource("/hello/{name}", |r| {
                r.method(http::Method::GET).with(example::say_hello_to_path);
            })
            .resource("/hello", |r| {
                r.method(http::Method::POST)
                    .with(example::say_hello_to_body);
            })
    }).bind(addr)
        .unwrap()
        .run();
}
