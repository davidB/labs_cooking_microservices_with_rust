# Router les requêtes

## Définir les méthodes retournant une réponse
Dans `reviews.rs`

```
use std::collections::HashMap;

use actix_web::{error, AsyncResponder, HttpResponse, Json, Path, State};
use futures::Future;
use futures;
```

### Définir comment extraire le product_id du path

```rust,no_run,ignore
#[derive(Deserialize)]
pub struct ProductId {
    product_id: i32,
}
```

### GET des reviews

```rust,no_run,ignore
pub fn reviews(
    product_id: Path<ProductId>,
    state: State<super::AppState>,
) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    let product_id = product_id.product_id;
    futures::future::ok(HttpResponse::Ok().finish()).responder()
}
```

### POST d'une review

```rust,no_run,ignore
#[derive(Debug, Deserialize, Serialize)]
pub struct NewReview {}

pub fn create_review(
    product_id: Path<ProductId>,
    review: Json<NewReview>,
    state: State<super::AppState>,
) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    let product_id = product_id.product_id;

    futures::future::ok(HttpResponse::Ok().finish()).responder()
}
```

## Définir les routes
Dans `lib.rs`

```rust,no_run,ignore
use actix_web::http;

mod reviews;
```

```rust,no_run,ignore
server::new(move || {
    App::with_state(AppState {
    }).middleware(Logger::default())
        .resource("/reviews/{product_id}", |r| {
            r.method(http::Method::GET).with2(reviews::reviews);
            r.method(http::Method::POST).with3(reviews::create_review);
        })
}).bind(addr)
    .unwrap()
    .start();
```