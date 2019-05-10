mod twitter_client;

#[macro_use(slog_o)]
extern crate slog;

extern crate slog_term;
extern crate slog_async;
extern crate slog_scope;
extern crate slog_stdlog;

use std::sync::Mutex;
use slog::Drain;

fn init_logger(filter_level: slog::Level) -> slog::Logger {
    let drain = Mutex::new(
                slog_term::FullFormat::new(
                    slog_term::TermDecorator::new()
                    .build()
                )
                    .use_local_timestamp()
                    .build()
                )
                    .filter_level(filter_level);

    slog::Logger::root(
        drain
        .fuse(),
        slog_o!(),
    )
}

fn main() {
    let config = twitter_client::Config::new().unwrap();
    let filter_level = match config.is_debug {
        false => slog::Level::Info,
        true => slog::Level::Debug,
    };
    let logger = init_logger(filter_level);
    let _scope_guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();
    slog_scope::scope(&slog_scope::logger().new(slog_o!("scope" => "1")), || {
        twitter_client::TwitterClient::new(config).watch();
    });
}