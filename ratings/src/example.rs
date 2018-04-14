use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::handler::{HandlerFuture, IntoHandlerError, IntoResponse};

use hyper::{Body, Response, StatusCode};
use futures::{future, Future, Stream};
use mime;

use serde_json;

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct NamePathExtractor {
    name: String,
}

pub fn router() -> Router {
    build_simple_router(|route| {
        route.post("/hello").to(say_hello_to_body);

        route
            .get("/goodbye/:name")
            .with_path_extractor::<NamePathExtractor>()
            .to(say_goodbye_to_path_with_struct);
    })
}

// serde attributes
// https://serde.rs/field-attrs.html
#[derive(Deserialize)]
struct Who {
    #[serde(rename = "say hello to")]
    name: String,
}

fn say_hello_to_body(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let res = match serde_json::from_slice::<Who>(&valid_body.to_vec()) {
                    Ok(who) => create_response(
                        &state,
                        StatusCode::Ok,
                        Some((
                            format!("Hello {}!", who.name).into_bytes(),
                            mime::TEXT_PLAIN,
                        )),
                    ),
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

fn say_goodbye_to_path_with_struct(state: State) -> (State, Message) {
    let res = {
        let who = NamePathExtractor::borrow_from(&state);
        Message {
            interjection: "Goodbye",
            name: who.name.clone(),
        }
    };

    (state, res)
}
