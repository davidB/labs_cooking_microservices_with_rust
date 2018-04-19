# Serveur HTTP

## Démarrer une application et un système d'acteur
Dans `lib.rs`

```rust,no_run,ignore
extern crate actix;
extern crate actix_web;
extern crate futures;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use actix_web::middleware::Logger;
use actix_web::{server, App};

pub struct AppState {
}
```

```rust,no_run,ignore
let sys = actix::System::new("reviews");

server::new(move || {
    App::with_state(AppState {
    }).middleware(Logger::default())
}).bind(addr)
    .unwrap()
    .start();

let _ = sys.run();
```
