mod twitter_client;

#[macro_use(slog_o,slog_b,slog_record,slog_record_static,slog_log,slog_trace,slog_debug,slog_info,slog_warn,slog_error,slog_crit,slog_kv)]
extern crate slog;

extern crate slog_term;
extern crate slog_scope;

use std::sync::Mutex;
use slog::Drain;

fn init_logger() -> slog::Logger {
    slog::Logger::root(
        Mutex::new(
            slog_term::FullFormat::new(
                slog_term::TermDecorator::new().build()
            ).build()
        ).fuse(), slog_o!(
            "version" => env!("CARGO_PKG_VERSION")
        )
    )
}

fn main() {
    let _scope_guard = slog_scope::set_global_logger(init_logger());
    slog_scope::scope(&slog_scope::logger().new(slog_o!("scope" => "1")), || {
        if let Ok(tc) = twitter_client::TwitterClient::new() {
            tc.watch();
        }
    });
}