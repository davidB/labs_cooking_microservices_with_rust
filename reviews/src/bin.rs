extern crate log;
/// Import longer-name versions of macros only to not collide with legacy `log`
#[macro_use(slog_info, slog_log, slog_o, slog_record, slog_record_static, slog_b, slog_kv)]
extern crate slog;
extern crate slog_async;
extern crate slog_envlogger;
extern crate slog_json;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;

extern crate reviews;

use slog::Drain;

mod config;

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

    let config = config::Config::new();
    let addr = format!("{}:{}", config.host, config.port);

    slog_info!(slog_scope::logger(), "slog info";"address" => &addr);
    slog_scope::scope(&slog_scope::logger().new(slog_o!("scope" => "1")), || {
        reviews::run(&addr)
    });
}
