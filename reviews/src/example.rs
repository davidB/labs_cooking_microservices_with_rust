use std;

use actix_web::client::ClientRequest;
use actix_web::{error, AsyncResponder, FutureResponse, HttpMessage, HttpResponse, Json, Path,
                Result, State};
use futures::future::Future;

use db;

#[derive(Deserialize)]
pub struct IdPathExtractor {
    id: i32,
}

#[derive(Serialize)]
struct Message {
    interjection: String,
    name: String,
}

pub fn say_hello(
    state: State<super::AppState>,
    hello: Path<IdPathExtractor>,
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(db::GetName(hello.id))
        .from_err()
        .and_then(|name| {
            Ok(HttpResponse::Ok().json(Message {
                interjection: "Hello".to_string(),
                name: name.unwrap_or_else(|_| "world".to_string()).clone(),
            }))
        })
        .responder()
}

pub fn say_random(
    state: State<super::AppState>,
    hello: Path<IdPathExtractor>,
) -> FutureResponse<HttpResponse> {
    ClientRequest::get("http://wordnik.com/randoml")
        .finish()
        .unwrap()
        .send()
        .map_err(error::Error::from)
        .and_then(|resp| {
            Ok(resp.headers()
                .get("location")
                .unwrap()
                .to_str()
                .unwrap()
                .split('/')
                .nth(2)
                .unwrap()
                .to_string())
        })
        .or_else(|err| {
            // in case of error, log it and continue with a default world
            error!("{:?}", err);
            Ok("hello".to_string())
        })
        .and_then(move |random| {
            state
                .db
                .send(db::GetName(hello.id))
                .from_err()
                .and_then(move |name| {
                    Ok(HttpResponse::Ok().json(Message {
                        interjection: random,
                        name: name.unwrap_or_else(|_| "world".to_string()).clone(),
                    }))
                })
        })
        .responder()
}

#[derive(Deserialize)]
pub struct Who {
    #[serde(rename = "say hello to")]
    name: String,
}

pub fn save_name(
    who: Json<Who>,
    state: State<super::AppState>,
    hello: Path<IdPathExtractor>,
) -> Result<HttpResponse> {
    state.db.do_send(db::SaveName {
        name: who.name.clone(),
        id: hello.id,
    });

    Ok(HttpResponse::Ok().finish())
}
