use std;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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
                let ratings = HashMap::new();
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

pub struct RatingsContext {
    product_id: i32,
    ratings: Arc<RwLock<Option<HashMap<String, i32>>>>,
}
impl RatingsContext {
    fn new(id: i32) -> Self {
        Self {
            product_id: id,
            ratings: Arc::new(RwLock::new(None)),
        }
    }
    fn fetch_ratings_if_needed(&self) -> () {
        let mut ratings = self.ratings.write().unwrap();
        if ratings.is_none() {
            *ratings = Some(
                reqwest::get(&format!(
                    "{}/ratings/{}",
                    ::CONFIG.ratings_url,
                    self.product_id
                )).and_then(|mut resp| resp.json())
                    .map(|ratings: reviews::RatingsResponse| ratings.ratings.reviewers)
                    .unwrap_or_else(|err| {
                        error!("{:?}", err);
                        HashMap::new()
                    }),
            );
        }
    }
    fn rating_by(&self, reviewer: &str) -> Option<i32> {
        self.fetch_ratings_if_needed();
        let ratings = self.ratings.read().unwrap();
        if let Some(ref ratings) = *ratings {
            let v = ratings.get(reviewer);
            if let Some(v) = v {
                Some(*v)
            } else {
                None
            }
        } else {
            None
        }
    }
}
impl juniper::Context for RatingsContext {}

impl juniper::GraphQLType for reviews::Product {
    type Context = ();
    type TypeInfo = ();

    fn name(_: &()) -> Option<&str> {
        Some("Product")
    }

    fn meta<'r>(_: &(), registry: &mut ::juniper::Registry<'r>) -> ::juniper::meta::MetaType<'r> {
        let fields = &[
            registry.field::<i32>("id", &()),
            registry.field::<Vec<reviews::Review>>("reviews", &()),
        ];
        let builder = registry.build_object_type::<reviews::Product>(&(), fields);
        builder.into_meta()
    }

    fn resolve_field(
        &self,
        _: &(),
        field_name: &str,
        _: &::juniper::Arguments,
        executor: &::juniper::Executor<Self::Context>,
    ) -> ::juniper::ExecutionResult {
        let ratings_context = RatingsContext::new(self.id);
        match field_name {
            "id" => executor.resolve_with_ctx(&(), &self.id),
            "reviews" => executor
                .replaced_context(&ratings_context)
                .resolve_with_ctx(&(), &self.reviews),
            _ => panic!("Field {} not found on my type {}", field_name, "Product"),
        }
    }
}

impl juniper::GraphQLType for reviews::Review {
    type Context = RatingsContext;
    type TypeInfo = ();

    fn name(_: &()) -> Option<&str> {
        Some("Review")
    }

    fn meta<'r>(_: &(), registry: &mut ::juniper::Registry<'r>) -> ::juniper::meta::MetaType<'r> {
        let fields = &[
            registry.field::<String>("reviewer", &()),
            registry.field::<String>("text", &()),
            registry.field::<Option<reviews::Rating>>("rating", &()),
        ];
        let builder = registry.build_object_type::<reviews::Review>(&(), fields);
        builder.into_meta()
    }

    fn resolve_field(
        &self,
        _: &(),
        field_name: &str,
        _: &::juniper::Arguments,
        executor: &::juniper::Executor<Self::Context>,
    ) -> ::juniper::ExecutionResult {
        let context = executor.context();
        match field_name {
            "reviewer" => executor.resolve_with_ctx(&(), &self.reviewer),
            "text" => executor.resolve_with_ctx(&(), &self.text),
            "rating" => executor.resolve_with_ctx(
                &(),
                &context
                    .rating_by(&self.reviewer)
                    .map(|v| reviews::rating_nb_to_rating(&v)),
            ),
            _ => panic!("Field {} not found on my type {}", field_name, "Review"),
        }
    }
}
