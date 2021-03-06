use std::env;

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT").unwrap_or_else(|_| "9080".to_string()),
        }
    }
}
