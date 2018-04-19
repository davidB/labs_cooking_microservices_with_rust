# Ajouter un healthcheck

## Nouvelle dépendance
Dans `lib.rs`

```rust,no_run
extern crate time;
```

## Module health
Dans `health.rs`

### Ajouter les imports

```rust,no_run
use actix_web::{HttpRequest, HttpResponse};

use time;
```

### Déclarer les données retournées

```rust,no_run
#[derive(Serialize)]
pub struct Healthcheck {
    now: i64,
    version: &'static str,
    status: &'static str,
}
```

### Retourner les données

```rust,no_run
pub fn healthcheck(_: HttpRequest<super::AppState>) -> HttpResponse {
    HttpResponse::Ok().json(Healthcheck {
        now: time::now_utc().to_timespec().sec,
        version: env!("CARGO_PKG_VERSION"),
        status: "Reviews is healthy",
    })
}
```

## Ajouter la route pour /GET healthcheck
Dans `lib.rs`

```rust,no_run
mod health;
```

```rust,no_run
    .resource("/health", |r| {
        r.method(http::Method::GET).f(health::healthcheck)
    })
```
