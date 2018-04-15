use gotham::http::response::create_response;
use gotham::state::State;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::handler::IntoResponse;

use hyper::{Response, StatusCode};
use mime;

use serde_json;

use time;

pub fn router() -> Router {
    build_simple_router(|route| {
        route.get("/health").to(healthcheck);
    })
}

#[derive(Serialize)]
struct Healthcheck {
    now: i64,
    version: &'static str,
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

fn healthcheck(state: State) -> (State, Healthcheck) {
    let res = Healthcheck {
        now: time::now_utc().to_timespec().sec,
        version: env!("CARGO_PKG_VERSION"),
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
