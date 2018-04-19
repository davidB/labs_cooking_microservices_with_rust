# Acteur DB

## Définir l'acteur
Dans `db.rs`

```rust,no_run,ignore
use actix::prelude::*;

pub struct DbExecutor();

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
```

### Définir le message `GetReviews`

```rust,no_run,ignore
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
        warn!("getting reviews for product {}", msg.product_id);

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

```rust,no_run,ignore
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
        warn!("saving review {:?}", msg.review);

        Ok(msg.review)
    }
}
```

## Ajouter l'acteur à l'`AppState`
Dans `lib.rs`

```rust,no_run,ignore
use actix::prelude::*;

mod db;

pub struct AppState {
    db: Addr<Syn, db::DbExecutor>,
}
```

```rust,no_run,ignore
let db_addr = SyncArbiter::start(3, move || db::DbExecutor());

server::new(move || {
    App::with_state(AppState {
        db: db_addr.clone(),
    }).middleware(Logger::default())
```

## Appeller l'acteur pendant les requêtes
Dans `reviews.rs`

```rust,no_run,ignore
use db;
```

### Get des reviews

Enlever
```rust,no_run,ignore
let reviews = vec![];
```

Et remplacer
```rust,no_run,ignore
futures::future::result({
```

par:
```rust,no_run,ignore
state
    .db
    .send(db::GetReviews {
        product_id: product_id,
    })
    .map(|result| match result {
        Ok(reviews) => reviews,
        Err(err) => {
            error!("{:?}", err);
            vec![]
        }
    })
    .from_err()
    .and_then(move |reviews| {
```

### Save d'une review

Remplacer 
```rust,no_run,ignore
futures::future::result({ Ok(HttpResponse::Ok().json(review.clone())) }).responder()
```

par:
```rust,no_run,ignore
state
    .db
    .send(db::SaveReview {
        review: review_to_save,
    })
    .from_err()
    .and_then(move |_| { Ok(HttpResponse::Ok().json(review.clone())) }).responder()
```

## Résultat

Notre avis s'affiche !
```
curl localhost:9080/reviews/0 -i
HTTP/1.1 200 OK
content-length: 57
content-type: application/json
date: Thu, 19 Apr 2018 22:13:43 GMT

{"id":0,"reviews":[{"reviewer":"user1","text":"great!"}]}
```

À la sauvegarde d'un avis, un log s'affiche:
```
curl localhost:9080/reviews/0 -i -H 'Content-Type: application/json' -d '{"reviewer":"moi","rating":3,"text":"mon avis"}'
HTTP/1.1 200 OK
content-length: 47
content-type: application/json
date: Thu, 19 Apr 2018 22:23:06 GMT

{"reviewer":"moi","text":"mon avis","rating":3}
```

```
{"msg":"saving review Review { product_id: 0, reviewer: \"moi\", review: \"mon avis\" }","level":"WARN","ts":"2018-04-20T00:23:06.973686+02:00","logger":"app"}
```
