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

use slog::{slog_error};
use slog_scope::{error};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
    track: String,
    slack_url: String,
    pub is_debug: bool,
    pub post_slack_enabled: bool,
    filter_lang: String,
}

pub const RESET_FLAG: u64 = 0;
pub const UNRESET_FLAG: u64 = 1;
pub const RETRY_FLAG: u64 = 2;

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
    pub fn new(cfg: &Config) -> Self {
        TwitterClient {
                config: cfg.to_owned(),
        }
    }

    pub fn watch(self) -> u64 {
        let consumer_key: &str = &self.config.consumer_key.replace("\n", "");
        let consumer_secret: &str = &self.config.consumer_secret.replace("\n", "");
        let access_token: &str = &self.config.access_token.replace("\n", "");
        let access_token_secret: &str = &self.config.access_token_secret.replace("\n", "");
        let track: &str = &self.config.track;
        let mut flag = RESET_FLAG;
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
                    flag = RESET_FLAG;
                    let lang = &tweet.lang.unwrap_or(std::borrow::Cow::Borrowed("none"));
                    let fileter_lang = &self.config.filter_lang;
                    if lang != fileter_lang && fileter_lang != "none" {
                        error!("this tweet lang is not {}, lang is {}, abort", fileter_lang, lang);
                        return Ok(())
                    }
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
            .map_err(move |e| {
                let msg: &str = &format!("{}", e);
                error!("{}", msg);
                match msg {
                    "420 <unknown status code>" => flag = UNRESET_FLAG,
                    _ => flag = RETRY_FLAG,
                }
            });

        rt::run(bot);
        flag
    }
}