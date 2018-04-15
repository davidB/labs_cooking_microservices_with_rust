use actix::prelude::*;

use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use schema;
use models;

pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Debug)]
pub struct GetReviews {
    pub product_id: Option<i32>,
}

impl Message for GetReviews {
    type Result = Result<Vec<models::Review>, diesel::result::Error>;
}

impl Handler<GetReviews> for DbExecutor {
    type Result = Result<Vec<models::Review>, diesel::result::Error>;

    fn handle(&mut self, msg: GetReviews, _: &mut Self::Context) -> Self::Result {
        use self::schema::reviews::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let mut item_query = reviews.into_boxed();

        if let Some(product_id_query) = msg.product_id {
            item_query = item_query.filter(product_id.eq(product_id_query))
        }

        let items = item_query.load::<models::Review>(conn)?;

        Ok(items)
    }
}

#[derive(Debug)]
pub struct SaveReview {
    pub review: models::Review,
}

impl Message for SaveReview {
    type Result = Result<models::Review, diesel::result::Error>;
}

impl Handler<SaveReview> for DbExecutor {
    type Result = Result<models::Review, diesel::result::Error>;

    fn handle(&mut self, msg: SaveReview, _: &mut Self::Context) -> Self::Result {
        use self::schema::reviews::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(reviews)
            .values(&msg.review)
            .execute(conn)?;

        Ok(msg.review)
    }
}
