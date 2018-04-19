# Router les requêtes

## Définir les méthodes retournant une réponse
Dans `reviews.rs`

```rust,no_run,ignore
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

    unimplemented!()
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

    unimplemented!()
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

## Résultat

Les routes définies ne répondent pas, et un log apparaît quand on les appelle

```bash
curl localhost:9080/reviews/0 -i
curl: (52) Empty reply from server
```

```
thread 'arbiter:"85373381-235b-47af-bde8-4d85e5b778d8":"actor"' panicked at 'not yet implemented', src/reviews.rs:18:5
```
