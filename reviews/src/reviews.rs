use std::collections::HashMap;

use actix_web::client::ClientRequest;
use actix_web::{error, AsyncResponder, HttpMessage, HttpResponse, Json, Path, State};
use futures::Future;

use db;
use models;

#[derive(Debug, Serialize)]
pub(crate) struct Product {
    pub(crate) id: i32,
    pub(crate) reviews: Vec<Review>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Color {
    Blue,
    Yellow,
    Red,
}

#[derive(Debug, Serialize)]
pub(crate) struct Rating {
    pub(crate) stars: i32,
    pub(crate) color: Color,
}

#[derive(Debug, Serialize)]
pub(crate) struct Review {
    pub(crate) reviewer: String,
    pub(crate) text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) rating: Option<Rating>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RatingsResponse {
    id: i32,
    pub(crate) ratings: HashMap<String, i32>,
}

#[derive(Deserialize)]
pub struct ProductId {
    product_id: i32,
}

pub(crate) fn rating_nb_to_rating(rating: &i32) -> Rating {
    Rating {
        stars: *rating,
        color: match *rating {
            1 => Color::Red,
            2 | 3 => Color::Yellow,
            _ => Color::Blue,
        },
    }
}

pub(crate) fn reviews_with_ratings(
    reviews: Vec<models::Review>,
    ratings: HashMap<String, i32>,
) -> Vec<Review> {
    //build the reviews
    reviews
        .iter()
        .map(|review| {
            let reviewer = review.reviewer.clone();
            Review {
                rating: ratings.get(&reviewer).map(rating_nb_to_rating),
                reviewer: reviewer,
                text: review.review.clone(),
            }
        })
        .collect()
}

pub(crate) fn reviews(
    product_id: Path<ProductId>,
    state: State<super::AppState>,
) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    let product_id = product_id.product_id;

    ClientRequest::get(&format!("{}/ratings/{}", ::CONFIG.ratings_url, product_id))
        .finish()
        .unwrap()
        .send()
        .map_err(error::Error::from)
        .and_then(move |resp| {
            resp.json()
                .from_err()
                .and_then(|ratings: RatingsResponse| Ok(ratings.ratings))
        })
        .or_else(|err| {
            // in case of error, log it and continue with an empty list of ratings
            error!("{:?}", err);
            Ok(HashMap::new())
        })
        .and_then(move |ratings| {
            state
                .db
                .send(db::GetReviews {
                    product_id: Some(product_id),
                })
                .from_err()
                .and_then(move |mut reviews| {
                    if reviews.is_err() {
                        error!("{:?}", reviews);
                        reviews = Ok(vec![]);
                    }
                    // build the response
                    let product = Product {
                        id: product_id,
                        reviews: reviews_with_ratings(reviews.unwrap(), ratings),
                    };
                    // return a 200 response with the reviews as json
                    Ok(HttpResponse::Ok().json(product))
                })
        })
        .responder()
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NewReview {
    reviewer: String,
    text: String,
    rating: i32,
}

pub(crate) fn create_review(
    product_id: Path<ProductId>,
    review: Json<NewReview>,
    state: State<super::AppState>,
) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    let product_id = product_id.product_id;

    let review_to_save = models::Review {
        product_id: product_id as i32,
        reviewer: review.reviewer.clone(),
        review: review.text.clone(),
    };

    ClientRequest::post(&format!("{}/ratings/{}", ::CONFIG.ratings_url, product_id))
        .json(review.0)
        .unwrap()
        .send()
        .map(|_| ())
        .or_else(|err| {
            // in case of error, log it and ignore it
            error!("{:?}", err);
            Ok(())
        })
        .and_then(move |_| {
            state
                .db
                .send(db::SaveReview {
                    review: review_to_save,
                })
                .from_err()
                .and_then(|res| match res {
                    Ok(review) => Ok(HttpResponse::Ok().json(review)),
                    Err(_) => Ok(HttpResponse::InternalServerError().into()),
                })
        })
        .responder()
}
