extern crate actix;
extern crate actix_web;
extern crate futures;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate diesel;
extern crate r2d2;

#[macro_use]
extern crate log;

extern crate time;

#[macro_use]
extern crate lazy_static;

extern crate itertools;
#[macro_use]
extern crate juniper;
extern crate reqwest;

use actix_web::middleware::{cors, Logger};
use actix_web::{http, server, App};
use actix::prelude::*;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

mod config;
mod health;
mod reviews;
mod db;
mod graphql;

mod models;
mod schema;

lazy_static! {
    static ref CONFIG: config::Config = config::Config::new();
}

pub struct AppState {
    db: Addr<Syn, db::DbExecutor>,
    graphql: Addr<Syn, graphql::GraphQLExecutor>,
}

pub fn run(addr: &str) {
    let sys = actix::System::new("reviews");

    let manager = ConnectionManager::<SqliteConnection>::new(CONFIG.database_url.clone());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let db_addr = SyncArbiter::start(3, move || db::DbExecutor(pool.clone()));

    // Start 3 graphql actors
    let schema = std::sync::Arc::new(graphql::create_schema());
    let graphql_addr = SyncArbiter::start(3, move || graphql::GraphQLExecutor::new(schema.clone()));

    server::new(move || {
        App::with_state(AppState {
            db: db_addr.clone(),
            graphql: graphql_addr.clone(),
        }).middleware(Logger::default())
            .resource("/health", |r| {
                r.method(http::Method::GET).f(health::healthcheck)
            })
            .resource("/reviews/{product_id}", |r| {
                r.method(http::Method::GET).with2(reviews::reviews);
                r.method(http::Method::POST).with3(reviews::create_review);
            })
            .resource("/graphql", |r| {
                cors::Cors::build()
                    .supports_credentials()
                    .finish()
                    .register(r);
                r.method(http::Method::POST).with2(graphql::graphql)
            })
            .resource("/graphiql", |r| {
                r.method(http::Method::GET).h(graphql::graphiql)
            })
    }).bind(addr)
        .unwrap()
        .start();

    let _ = sys.run();
}
