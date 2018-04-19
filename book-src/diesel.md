# Utilisation de diesel

## Nouvelles dépendances
Dans `lib.rs`

```rust,no_run
#[macro_use]
extern crate diesel;
extern crate r2d2;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
```

## Migrations et schémas
Dans `lib.rs`

...

```rust,no_run
mod schema;
```

## Ajout d'attributs au model
Dans `model.rs`

```rust,no_run
use super::schema::reviews;

#[derive(Serialize, Debug, Queryable, Insertable)]
#[table_name = "reviews"]
pub struct Review {
    pub product_id: i32,
    pub reviewer: String,
    pub review: String,
}
```

## Configurer le pool de connection
Dans `lib.rs`

```rust,no_run
let manager = ConnectionManager::<SqliteConnection>::new(CONFIG.database_url.clone());
let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
let db_addr = SyncArbiter::start(3, move || db::DbExecutor(pool.clone()));
```

## Acteur db `db.rs`

### Importer les nouvelles dépendances

```rust,no_run
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use schema;
```

### Ajouter le pool de connection

```rust,no_run
pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);
```

### Récupérer les reviews

```rust,no_run
#[derive(Debug)]
pub struct GetReviews {
    pub product_id: i32,
}

impl Message for GetReviews {
    type Result = Result<Vec<models::Review>, diesel::result::Error>;
}

impl Handler<GetReviews> for DbExecutor {
    type Result = Result<Vec<models::Review>, diesel::result::Error>;

    fn handle(&mut self, msg: GetReviews, _: &mut Self::Context) -> Self::Result {
        info!("getting reviews for product {}", msg.product_id);

        use self::schema::reviews::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let items = reviews.filter(product_id.eq(msg.product_id)).load::models::Review(conn)?;

        Ok(items)
    }
}
```

### Sauvegarder une review

```rust,no_run
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
        info!("saving review {:?}", msg.review);

        use self::schema::reviews::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(reviews)
            .values(&msg.review)
            .execute(conn)?;

        Ok(msg.review)
    }
}
```
