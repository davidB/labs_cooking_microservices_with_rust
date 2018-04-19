# Acteur DB

## Définir l'acteur
Dans `db.rs`

```rust,no_run
use actix::prelude::*;

pub struct DbExecutor();

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
```

### Définir le message `GetReviews`

```rust,no_run
use models;

#[derive(Debug)]
pub struct GetReviews {
    pub product_id: i32,
}

impl Message for GetReviews {
    type Result = Result<Vec<models::Review>, ()>;
}

impl Handler<GetReviews> for DbExecutor {
    type Result = Result<Vec<models::Review>, ()>;

    fn handle(&mut self, msg: GetReviews, _: &mut Self::Context) -> Self::Result {
        info!("getting reviews for product {}", msg.product_id);

        Ok(vec![
            models::Review {
                product_id: msg.product_id,
                review: "great!".to_string(),
                reviewer: "user1".to_string(),
            },
        ])
    }
}
```

### Définir le message `SaveReview`

```rust,no_run
#[derive(Debug)]
pub struct SaveReview {
    pub review: models::Review,
}

impl Message for SaveReview {
    type Result = Result<models::Review, ()>;
}

impl Handler<SaveReview> for DbExecutor {
    type Result = Result<models::Review, ()>;

    fn handle(&mut self, msg: SaveReview, _: &mut Self::Context) -> Self::Result {
        info!("saving review {:?}", msg.review);

        Ok(msg.review)
    }
}
```

## Ajouter l'acteur à l'`AppState`
Dans `lib.rs`

```rust,no_run
use actix::prelude::*;

mod db;

pub struct AppState {
    db: Addr<Syn, db::DbExecutor>,
}
```

```rust,no_run
let db_addr = SyncArbiter::start(3, move || db::DbExecutor());

server::new(move || {
    App::with_state(AppState {
        db: db_addr.clone(),
    }).middleware(Logger::default())
```

## Appeller l'acteur pendant les requêtes
Dans `reviews.rs`

```rust,no_run
use db;
```

### Get des reviews

```rust,no_run
state
    .db
    .send(db::GetReviews {
        product_id: product_id,
    })
    .and_then(move |mut reviews| {
        ...
```

### Save d'une review

```rust,no_run
state
    .db
    .send(db::SaveReview {
        review: review_to_save,
    })
    .from_err()
    .and_then(|res| match res {
        ...
```
