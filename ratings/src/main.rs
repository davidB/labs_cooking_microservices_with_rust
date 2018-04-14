extern crate gotham;
#[macro_use]
extern crate gotham_derive;

extern crate futures;
extern crate hyper;
extern crate mime;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::handler::{HandlerFuture, IntoHandlerError};

use hyper::{Body, Response, StatusCode};
use futures::{future, Future, Stream};

// Extract from the path
// https://github.com/gotham-rs/gotham/blob/master/examples/path/introduction
#[derive(Deserialize, StateData, StaticResponseExtender)]
struct NamePathExtractor {
    name: String,
}

// Building a router
// https://github.com/gotham-rs/gotham/tree/master/examples/routing/http_verbs
fn router() -> Router {
    build_simple_router(|route| {
        route.get("/hello").to(say_hello_world);

        route
            .get("/hello/:name")
            .with_path_extractor::<NamePathExtractor>()
            .to(say_hello_to_path);

        route.post("/hello").to(say_hello_to_body);
    })
}

fn say_hello_world(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Hello World!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

fn say_hello_to_path(state: State) -> (State, Response) {
    let res = {
        let who = NamePathExtractor::borrow_from(&state);
        create_response(
            &state,
            StatusCode::Ok,
            Some((
                format!("Hello {}!", who.name).into_bytes(),
                mime::TEXT_PLAIN,
            )),
        )
    };

    (state, res)
}

// Read the body
// https://github.com/gotham-rs/gotham/tree/master/examples/handlers/request_data
fn say_hello_to_body(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                let res = create_response(
                    &state,
                    StatusCode::Ok,
                    Some((
                        format!("Hello {}!", body_content).into_bytes(),
                        mime::TEXT_PLAIN,
                    )),
                );
                future::ok((state, res))
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
