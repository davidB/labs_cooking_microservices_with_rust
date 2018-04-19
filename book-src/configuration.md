# Configuration

## module `config.rs`

```rust,no_run,ignore
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
            port: env::var("PORT").unwrap_or_else(|_| "9081".to_string()),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "db.sqlite".to_string()),
            ratings_url: env::var("RATINGS_URL")
                .unwrap_or_else(|_| "http://ratings:9080".to_string()),
        }
    }
}
```

## Ecouter sur l'adresse de la configuration
Dans `bin.rs`

```rust,no_run,ignore
mod config;
```

Remplacer l'assignation existante d'`addr` par:
```
let config = config::Config::new();
let addr = format!("{}:{}", config.host, config.port);
```

## Rendre la configuration accessible
Dans `lib.rs`

```rust,no_run,ignore
#[macro_use]
extern crate lazy_static;

mod config;

lazy_static! {
    static ref CONFIG: config::Config = config::Config::new();
}
```

## Résultat

Notre service écoute maintenant sur le port 9081
```
curl localhost:9081/reviews/0 -i
HTTP/1.1 200 OK
content-length: 57
content-type: application/json
date: Thu, 19 Apr 2018 22:27:26 GMT

{"id":0,"reviews":[{"reviewer":"user1","text":"great!"}]}
```
