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

pub fn start() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, example::router())
}
