use std;

use futures::Future;

use serde_json;

use actix::prelude::*;
use actix_web::{AsyncResponder, Error, FutureResponse, HttpRequest, HttpResponse, Json, State};

use juniper;
use juniper::http::GraphQLRequest;
use juniper::http::graphiql::graphiql_source;
use juniper::EmptyMutation;

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
        let products = vec![
            reviews::Product {
                id: 0,
                reviews: vec![
                    reviews::Review {
                        reviewer: "Reviewer1".to_string(),
                        text: "An extremely entertaining play by Shakespeare. The slapstick humour is refreshing!".to_string(),
                        rating: Some(reviews::Rating {
                            stars: 5,
                            color: reviews::Color::Blue,
                        }),
                    },
                    reviews::Review {
                        reviewer: "Reviewer2".to_string(),
                        text: "Absolutely fun and entertaining. The play lacks thematic depth when compared to other plays by Shakespeare.".to_string(),
                        rating: Some(reviews::Rating {
                            stars: 4,
                            color: reviews::Color::Blue,
                        }),
                    },
                ]
            }
        ];

        Ok(products)
    }
});

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}