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

use std::sync::{Arc, Mutex};

use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::handler::{HandlerFuture, IntoHandlerError, IntoResponse};

use hyper::{Body, Response, StatusCode};
use futures::{future, Future, Stream};

lazy_static! {
    static ref NAME: Arc<Mutex<String>> = { Arc::new(Mutex::new("world".to_string())) };
}

fn router() -> Router {
    build_simple_router(|route| {
        route.post("/hello").to(save_who_to_say_hello_to);

        route.get("/hello").to(say_hello_to);
    })
}

#[derive(Deserialize)]
struct Who {
    #[serde(rename = "say hello to")]
    name: String,
}

fn save_who_to_say_hello_to(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let res = match serde_json::from_slice::<Who>(&valid_body.to_vec()) {
                    Ok(who) => {
                        let mut name = NAME.lock().unwrap();
                        *name = who.name.clone();
                        create_response(&state, StatusCode::Ok, None)
                    }
                    Err(_) => create_response(&state, StatusCode::BadRequest, None),
                };
                future::ok((state, res))
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}

#[derive(Serialize)]
struct Message {
    interjection: &'static str,
    name: String,
}

impl IntoResponse for Message {
    fn into_response(self, state: &State) -> Response {
        create_response(
            state,
            StatusCode::Ok,
            Some((
                serde_json::to_string(&self)
                    .expect("serialized Message")
                    .into_bytes(),
                mime::APPLICATION_JSON,
            )),
        )
    }
}

fn say_hello_to(state: State) -> (State, Message) {
    let res = Message {
        interjection: "Hello",
        name: NAME.lock().unwrap().clone(),
    };

    (state, res)
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
