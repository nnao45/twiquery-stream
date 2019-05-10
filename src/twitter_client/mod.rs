mod exec;

extern crate tweetust;
extern crate twitter_stream;
extern crate twitter_stream_message;
extern crate chrono;
extern crate snailquote;
extern crate curl;
extern crate slog_scope;
extern crate tokio_timer;

use twitter_stream::{TwitterStreamBuilder};
use twitter_stream::rt::{self, Future, Stream};
use twitter_stream_message::StreamMessage;

use chrono::Local;

use snailquote::unescape;
use exec as Exec;

use serde::Deserialize;

use slog::{slog_error,slog_debug};
use slog_scope::{error,debug};

use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct Config {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
    track: String,
    slack_url: String,
    pub is_debug: bool,
    pub post_slack_enabled: bool,
}

impl Config {
    pub fn new() -> Result<Self, ()> {
        match envy::from_env::<Config>() {
            Ok(config) => Ok(config),
            Err(e) => panic!("{:#?}", e),
        }
    }
}

pub struct TwitterClient {
    pub config: Config,
}

impl TwitterClient {
    pub fn new(cfg: Config) -> Self {
        TwitterClient {
                config: cfg,
        }
    }

    pub fn watch(self) {
        let consumer_key: &str = &self.config.consumer_key;
        let consumer_secret: &str = &self.config.consumer_secret;
        let access_token: &str = &self.config.access_token;
        let access_token_secret: &str = &self.config.access_token_secret;
        let track: &str = &self.config.track;
        let bot = TwitterStreamBuilder::filter(twitter_stream::Token::new(
                    consumer_key,
                    consumer_secret,
                    access_token,
                    access_token_secret,
                ))
            .track(Some(track))
            .listen()
            .unwrap()
            .flatten_stream()
            .for_each(move |json| {
                if let Ok(StreamMessage::Tweet(tweet)) = StreamMessage::from_str(&json) {
                    Exec::Executer::new(
                        &self.config.slack_url,
                        self.config.post_slack_enabled,
                        Exec::TweiqueryData::new(
                            &self.config.track,
                            &format!("{}", &tweet.user.name)[..],
                            &format!("{}", &tweet.user.screen_name)[..],
                            &unescape(&format!("{:?}", &tweet.text)).unwrap()[..],
                            &format!("{}",tweet.created_at.with_timezone(&Local))[..],
                            &format!("{}", &tweet.id)[..],
                        ),
                    )
                    .exec();
                }
                Ok(())
            })
            .map_err(|e| {
                let msg = format!("{}", e);
                error!("stream error: {}", msg);

                let mut base_timeout = 30;
                let mut clj = move || {
                    base_timeout *= 2;
                    if base_timeout == 60 * 60 {
                        std::process::exit(1)
                    }
                    base_timeout
                };
                if msg == "420 <unknown status code>" {
                    let sleep_time = clj();
                    debug!("stream api return 420, sleep {}", sleep_time);
                    std::thread::sleep(Duration::from_secs(sleep_time))
                }
            });

        rt::run(bot);
    }
}
