extern crate actix;
extern crate actix_web;
extern crate futures;

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
use actix::prelude::*;

mod config;
mod health;
mod example;
mod db;

lazy_static! {
    static ref CONFIG: config::Config = config::Config::new();
}

pub struct AppState {
    db: Addr<Syn, db::DbExecutor>,
}

pub fn run(addr: &str) {
    let sys = actix::System::new("reviews");

    let db_addr = SyncArbiter::start(1, move || db::DbExecutor {
        name: "world".to_string(),
    });

    server::new(move || {
        App::with_state(AppState {
            db: db_addr.clone(),
        }).middleware(Logger::default())
            .resource("/health", |r| {
                r.method(http::Method::GET).f(health::healthcheck)
            })
            .resource("/hello", |r| {
                r.method(http::Method::GET).with(example::say_hello);
                r.method(http::Method::POST).with2(example::save_name);
            })
    }).bind(addr)
        .unwrap()
        .start();

    let _ = sys.run();
}
