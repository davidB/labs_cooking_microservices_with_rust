extern crate actix;
extern crate actix_web;
extern crate futures;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;
extern crate r2d2;

#[macro_use]
extern crate log;

extern crate time;

#[macro_use]
extern crate lazy_static;

use actix_web::middleware::Logger;
use actix_web::{http, server, App};
use actix::prelude::*;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

mod config;
mod health;
mod example;
mod db;

mod models;
mod schema;

lazy_static! {
    static ref CONFIG: config::Config = config::Config::new();
}

pub struct AppState {
    db: Addr<Syn, db::DbExecutor>,
}

pub fn run(addr: &str) {
    let sys = actix::System::new("reviews");

    let manager = ConnectionManager::<SqliteConnection>::new(CONFIG.database_url.clone());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let db_addr = SyncArbiter::start(3, move || db::DbExecutor(pool.clone()));

    server::new(move || {
        App::with_state(AppState {
            db: db_addr.clone(),
        }).middleware(Logger::default())
            .resource("/health", |r| {
                r.method(http::Method::GET).f(health::healthcheck)
            })
            .resource("/hello/{id}", |r| {
                r.method(http::Method::GET).with2(example::say_hello);
                r.method(http::Method::POST).with3(example::save_name);
            })
            .resource("/random/{id}", |r| {
                r.method(http::Method::GET).with2(example::say_random);
            })
    }).bind(addr)
        .unwrap()
        .start();

    let _ = sys.run();
}
