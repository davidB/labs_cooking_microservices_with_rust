use actix_web::{HttpRequest, HttpResponse};

use time;

#[derive(Serialize)]
pub struct Healthcheck {
    now: i64,
    version: &'static str,
    status: &'static str,
}

pub fn healthcheck(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(Healthcheck {
        now: time::now_utc().to_timespec().sec,
        version: env!("CARGO_PKG_VERSION"),
        status: "Reviews is healthy",
    })
}
