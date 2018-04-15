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
pub struct GetName(pub i32);

impl Message for GetName {
    type Result = Result<String, diesel::result::Error>;
}

impl Handler<GetName> for DbExecutor {
    type Result = Result<String, diesel::result::Error>;

    fn handle(&mut self, msg: GetName, _: &mut Self::Context) -> Self::Result {
        use self::schema::hello::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let item = hello
            .filter(hello_id.eq(msg.0))
            .first::<models::Hello>(conn)?;
        Ok(item.name)
    }
}

#[derive(Debug)]
pub struct SaveName {
    pub id: i32,
    pub name: String,
}

impl Message for SaveName {
    type Result = Result<(), diesel::result::Error>;
}

impl Handler<SaveName> for DbExecutor {
    type Result = Result<(), diesel::result::Error>;

    fn handle(&mut self, msg: SaveName, _: &mut Self::Context) -> Self::Result {
        info!("saving {:?}", msg);

        use self::schema::hello::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(hello)
            .values(&models::Hello {
                hello_id: msg.id,
                name: msg.name,
            })
            .execute(conn)?;

        Ok(())
    }
}
