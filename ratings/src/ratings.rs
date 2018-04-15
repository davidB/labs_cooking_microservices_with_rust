use std::collections::HashMap;

use hyper::{Body, Response, StatusCode};
use mime;
use futures::{future, Future, Stream};

use serde_json;

use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::handler::{HandlerFuture, IntoHandlerError, IntoResponse};

use db;

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct ProductIdPathExtractor {
    product_id: u32,
}

#[derive(Debug, Serialize)]
pub struct RatingsResponse {
    id: u32,
    ratings: RatingsPerUser,
}

#[derive(Debug, Serialize)]
pub struct RatingsPerUser {
    #[serde(flatten)]
    ratings: HashMap<String, u8>,
}

impl IntoResponse for RatingsResponse {
    fn into_response(self, state: &State) -> Response {
        create_response(
            state,
            StatusCode::Ok,
            Some((
                serde_json::to_string(&self)
                    .expect("serialized Ratings")
                    .into_bytes(),
                mime::APPLICATION_JSON,
            )),
        )
    }
}

pub fn ratings(state: State) -> (State, RatingsResponse) {
    let res = {
        let product = ProductIdPathExtractor::borrow_from(&state);
        info!("requesting ratings for product {}", product.product_id);
        let ratings = db::RATINGS
            .read()
            .unwrap()
            .iter()
            .filter(|rating| rating.product_id == product.product_id)
            .map(|rating| (rating.reviewer.clone(), rating.rating))
            .collect();
        RatingsResponse {
            id: product.product_id,
            ratings: RatingsPerUser { ratings },
        }
    };

    (state, res)
}

#[derive(Deserialize, Debug)]
struct NewRating {
    reviewer: String,
    rating: u8,
}

pub fn save_rating(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let res = match serde_json::from_slice::<NewRating>(&valid_body.to_vec()) {
                    Ok(rating) => {
                        let product = ProductIdPathExtractor::borrow_from(&state);
                        info!(
                            "saving new rating {:?} for product {}",
                            rating, product.product_id
                        );
                        db::RATINGS.write().unwrap().push(db::Rating {
                            product_id: ProductIdPathExtractor::borrow_from(&state).product_id,
                            reviewer: rating.reviewer,
                            rating: rating.rating,
                        });
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
