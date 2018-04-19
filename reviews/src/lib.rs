#[macro_use]
extern crate log;

pub fn run(addr: &str) {
    warn!("listening on {}", addr);
}
