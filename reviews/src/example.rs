use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, Result, State, Path};
use futures::future::Future;

use db;

#[derive(Deserialize)]
pub struct IdPathExtractor {
    id: i32,
}

#[derive(Serialize)]
struct Message {
    interjection: &'static str,
    name: String,
}

pub fn say_hello(state: State<super::AppState>, hello: Path<IdPathExtractor>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(db::GetName(hello.id))
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

pub fn save_name(who: Json<Who>, state: State<super::AppState>, hello: Path<IdPathExtractor>) -> Result<HttpResponse> {
    state.db.do_send(db::SaveName {
        name: who.name.clone(),
        id: hello.id
    });

    Ok(HttpResponse::Ok().finish())
}
