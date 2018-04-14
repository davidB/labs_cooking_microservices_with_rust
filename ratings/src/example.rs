use std::sync::{Arc, Mutex};

use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::handler::{HandlerFuture, IntoHandlerError, IntoResponse};

use hyper::{Body, Response, StatusCode};
use futures::{future, Future, Stream};
use mime;

use serde_json;

lazy_static! {
    static ref NAME: Arc<Mutex<String>> = { Arc::new(Mutex::new("world".to_string())) };
}

pub fn router() -> Router {
    build_simple_router(|route| {
        route.post("/hello").to(save_who_to_say_hello_to);

        route.get("/hello").to(say_hello_to);
    })
}

/// # Examples
///
/// ```
/// # extern crate serde_json;
/// # extern crate ratings;
/// # use ratings::example::Who;
/// # fn main() {
///     let res = serde_json::from_str::<Who>("{\"say hello to\": \"Rust\"}");
/// #    assert!(res.is_ok());
/// # }
/// ```
#[derive(Deserialize)]
pub struct Who {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deserialize() {
        let res = serde_json::from_str::<Who>("{\"say hello to\": \"Rust\"}");

        assert!(res.is_ok());
    }
}