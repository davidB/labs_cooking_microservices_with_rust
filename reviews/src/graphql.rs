use std;
use std::collections::HashMap;

use futures::Future;

use serde_json;

use actix::prelude::*;
use actix_web::{AsyncResponder, Error, FutureResponse, HttpRequest, HttpResponse, Json, State};

use juniper;
use juniper::http::GraphQLRequest;
use juniper::http::graphiql::graphiql_source;
use juniper::EmptyMutation;

use itertools::Itertools;
use reqwest;

use db;
use reviews;

pub struct GraphQLExecutor {
    schema: std::sync::Arc<Schema>,
}

impl GraphQLExecutor {
    pub fn new(schema: std::sync::Arc<Schema>) -> GraphQLExecutor {
        GraphQLExecutor { schema: schema }
    }
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

pub struct GraphQLExecuteOnDB {
    request: GraphQLRequest,
    db: Addr<Syn, db::DbExecutor>,
}

impl Message for GraphQLExecuteOnDB {
    type Result = Result<String, Error>;
}

impl Handler<GraphQLExecuteOnDB> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLExecuteOnDB, _: &mut Self::Context) -> Self::Result {
        let res = msg.request.execute(&self.schema, &Context(msg.db));
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}

pub fn graphiql(_req: HttpRequest<super::AppState>) -> Result<HttpResponse, Error> {
    let html = graphiql_source(&format!("http://127.0.0.1:{}/graphql", super::CONFIG.port));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub fn graphql(
    st: State<super::AppState>,
    data: Json<GraphQLRequest>,
) -> FutureResponse<HttpResponse> {
    st.graphql
        .send(GraphQLExecuteOnDB {
            request: data.0,
            db: st.db.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(object) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(object)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

use juniper::FieldResult;
use juniper::RootNode;

pub struct QueryRoot;

pub struct Context(Addr<Syn, db::DbExecutor>);

impl juniper::Context for Context {}

graphql_object!(QueryRoot: Context |&self| {
    field products(&executor, id: Option<i32>) -> FieldResult<Vec<reviews::Product>> {
        let context = executor.context();

        let reviews = context.0.send(db::GetReviews {
            product_id: id
        }).wait().unwrap().unwrap();

        let products = reviews.into_iter()
            .group_by(|review| review.product_id).into_iter()
            .map(|(product_id, reviews_of_product)| {
                let ratings = reqwest::get(&format!(
                    "{}/ratings/{}",
                    ::CONFIG.ratings_url,
                    product_id
                )).and_then(|mut resp| resp.json())
                    .map(|ratings: reviews::RatingsResponse| ratings.ratings.reviewers)
                    .unwrap_or_else(|err| {
                        error!("{:?}", err);
                        HashMap::new()
                    });
                reviews::Product {
                    id: product_id,
                    reviews: reviews::reviews_with_ratings(
                        reviews_of_product.collect(),
                        ratings
                    ),
                }
            }).collect();

        Ok(products)
    }
});

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
