use gotham::http::response::create_response;
use gotham::state::State;
use gotham::handler::IntoResponse;

use hyper::{Response, StatusCode};
use mime;

use serde_json;

use time;

#[derive(Serialize)]
pub struct Healthcheck {
    now: i64,
    version: &'static str,
    status: &'static str,
}

impl IntoResponse for Healthcheck {
    fn into_response(self, state: &State) -> Response {
        create_response(
            state,
            StatusCode::Ok,
            Some((
                serde_json::to_string(&self)
                    .expect("serialized Healthcheck")
                    .into_bytes(),
                mime::APPLICATION_JSON,
            )),
        )
    }
}

pub fn healthcheck(state: State) -> (State, Healthcheck) {
    let res = Healthcheck {
        now: time::now_utc().to_timespec().sec,
        version: env!("CARGO_PKG_VERSION"),
        status: "Ratings is healthy",
    };

    (state, res)
}
