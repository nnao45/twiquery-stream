mod twitter_client;

#[macro_use(slog_o)]
extern crate slog;

extern crate slog_term;
extern crate slog_async;
extern crate slog_scope;
extern crate slog_stdlog;

use std::sync::Mutex;
use slog::Drain;

use slog::{slog_error};
use slog_scope::{error};

use std::time::Duration;

use twitter_client::Streamer;

fn init_logger(filter_level: slog::Level) -> slog::Logger {
    let drain = Mutex::new(slog_term::FullFormat::new(slog_term::TermDecorator::new().build())
                    .use_local_timestamp()
                    .build())
                    .filter_level(filter_level);

    slog::Logger::root(
        drain
        .fuse(),
        slog_o!(),
    )
}

fn main() {
    let config: twitter_client::Config = twitter_client::Config::new().unwrap();
    let filter_level = match config.is_debug {
        false => slog::Level::Info,
        true => slog::Level::Debug,
    };
    let logger = init_logger(filter_level);
    let _scope_guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();
    slog_scope::scope(&slog_scope::logger().new(slog_o!("scope" => "1")), || {
        let base_timeout = 30;
        let mut counter = create_counter(base_timeout);
        loop {
            match twitter_client::TwitterClient::new(&config).watch(twitter_client::TwitterStreamer::new()) {
                twitter_client::RESET_FLAG =>  counter = create_counter(base_timeout),
                twitter_client::UNRESET_FLAG => (),
            };
            std::thread::sleep(Duration::from_secs(counter()));
        }
    });
}

fn create_counter(mut base_timeout :u64) -> Box<FnMut() -> u64> {
// 変数xはスタック上に確保されるので、moveを使ってxのコピーの所有権をクロージャに移してあげる。
    let clj = move || {
        base_timeout *= 2;
        if base_timeout == 60 * 60 {
            std::process::exit(1)
        }
        error!("stream api error return, sleep {}", &base_timeout);
        base_timeout
    };

    // Rustは、クロージャを返す場合にはBoxで包んで返して上げる必要がある。
    Box::new(clj)
}