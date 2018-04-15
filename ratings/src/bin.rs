#[macro_use]
extern crate log;
/// Import longer-name versions of macros only to not collide with legacy `log`
#[macro_use(slog_error, slog_warn, slog_info, slog_debug, slog_trace, slog_log, slog_o,
            slog_record, slog_record_static, slog_b, slog_kv)]
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

fn demo_log() {
    slog_error!(slog_scope::logger(), "slog error"; "k1" => 1, "k2" => "v2");
    slog_warn!(slog_scope::logger(), "slog warn");
    slog_info!(slog_scope::logger(), "slog info");
    slog_debug!(slog_scope::logger(), "slog {}", "debug");
    slog_trace!(slog_scope::logger(), "slog {}", "trace");

    error!("log error");
    warn!("log warn");
    info!("log info");
    debug!("log {}", "debug");
    trace!("log {}", "trace");
}

fn main() {
    let root_logger = init_log();
    let _scope_guard = slog_scope::set_global_logger(root_logger);
    let _log_guard = slog_stdlog::init().unwrap();

    demo_log();

    ratings::start();
}
