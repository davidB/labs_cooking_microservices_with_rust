use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, Result, State};
use futures::future::Future;

use db;

#[derive(Serialize)]
struct Message {
    interjection: &'static str,
    name: String,
}

pub fn say_hello(state: State<super::AppState>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(db::GetName)
        .from_err()
        .and_then(|name| {
            Ok(HttpResponse::Ok().json(Message {
                interjection: "Hello",
                name: name.unwrap_or_else(|_| "world".to_string()).clone(),
            }))
        })
        .responder()
}

#[derive(Deserialize)]
pub struct Who {
    #[serde(rename = "say hello to")]
    name: String,
}

pub fn save_name(who: Json<Who>, state: State<super::AppState>) -> Result<HttpResponse> {
    state.db.do_send(db::SaveName {
        name: who.name.clone(),
    });

    Ok(HttpResponse::Ok().finish())
}
