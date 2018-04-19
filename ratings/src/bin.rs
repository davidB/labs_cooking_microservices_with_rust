extern crate log;
#[macro_use(slog_o, slog_kv)]
extern crate slog;
extern crate slog_async;
extern crate slog_envlogger;
extern crate slog_json;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;

extern crate ratings;

use slog::Drain;

fn init_log() -> slog::Logger {
    // format
    let drain = slog_json::Json::default(std::io::stderr()).fuse();

    // configuration
    let drain = slog_envlogger::new(drain);

    // synchronization
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, slog_o!("logger" => "app"))
}

fn main() {
    let root_logger = init_log();
    let _scope_guard = slog_scope::set_global_logger(root_logger);
    let _log_guard = slog_stdlog::init().unwrap();

    slog_scope::scope(&slog_scope::logger().new(slog_o!("scope" => "1")), || {
        ratings::start()
    });
}
