# Configuration

## module `config.rs`

```rust,no_run
use std::env;

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub database_url: String,
    pub ratings_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT").unwrap_or_else(|_| "9080".to_string()),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "db.sqlite".to_string()),
            ratings_url: env::var("RATINGS_URL")
                .unwrap_or_else(|_| "http://ratings:9080".to_string()),
        }
    }
}
```

## Ecouter sur l'adresse de la configuration
Dans `bin.rs`

```rust,no_run
mod config;
```

```
let config = config::Config::new();
let addr = format!("{}:{}", config.host, config.port);
```

## Rendre la configuration accessible
Dans `lib.rs`

```rust,no_run
#[macro_use]
extern crate lazy_static;

mod config;

lazy_static! {
    static ref CONFIG: config::Config = config::Config::new();
}
```