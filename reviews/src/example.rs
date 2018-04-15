use actix_web::{HttpResponse, Json, Path, Result};

#[derive(Deserialize)]
pub struct NamePathExtractor {
    name: String,
}

#[derive(Serialize)]
struct Message {
    interjection: &'static str,
    name: String,
}

pub fn say_hello_to_path(who: Path<NamePathExtractor>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Message {
        interjection: "Hello",
        name: who.name.clone(),
    }))
}

#[derive(Deserialize)]
pub struct Who {
    #[serde(rename = "say hello to")]
    name: String,
}

pub fn say_hello_to_body(who: Json<Who>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Message {
        interjection: "Hello",
        name: who.name.clone(),
    }))
}
