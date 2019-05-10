mod exec;

extern crate yaml_rust;
extern crate tweetust;
extern crate twitter_stream;
extern crate twitter_stream_message;
extern crate chrono;
extern crate snailquote;
extern crate curl;
extern crate slog_scope;

use curl::easy::Easy;
use curl::Error as CurlError;

use yaml_rust::{Yaml, YamlLoader};

use twitter_stream::{TwitterStreamBuilder};
use twitter_stream::rt::{self, Future, Stream};
use twitter_stream_message::StreamMessage;

use std::fs;
use std::io::{BufRead, BufReader, Error};
use chrono::Local;

use snailquote::unescape;
use exec as Exec;

use serde::Deserialize;

use slog::slog_crit;

pub struct TwitterClient {
    config: Config,
}

#[derive(Deserialize, Debug, Clone)]
struct Config {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
    track: String,
    slack_url: String,
}

impl TwitterClient {
    pub fn new() -> Result<Self, ()> {
        let load_env = |config: Config| -> Self {
            TwitterClient {
                config: config,
            }
        };
        match envy::from_env::<Config>() {
            Ok(config) => Ok(load_env(config)),
            Err(e) => Err(slog_crit!(slog_scope::logger(), "{:#?}", e)),
        }
    }

    pub fn watch(self) {
        let consumer_key: &str = &self.config.consumer_key;
        let consumer_secret: &str = &self.config.consumer_secret;
        let access_token: &str = &self.config.access_token;
        let access_token_secret: &str = &self.config.access_token_secret;
        let t: &str = &self.config.track;
        let bot = TwitterStreamBuilder::filter(twitter_stream::Token::new(
                    consumer_key,
                    consumer_secret,
                    access_token,
                    access_token_secret,
                ))
            .track(Some(t))
            .listen()
            .unwrap()
            .flatten_stream()
            .for_each(move |json| {
                if let Ok(StreamMessage::Tweet(tweet)) = StreamMessage::from_str(&json) {
                    match Exec::Executer::new(
                        &self.config.slack_url,
                        Exec::TweiqueryData::new(
                            &self.config.track,
                            &format!("{}", &tweet.user.name)[..],
                            &format!("{}", &tweet.user.screen_name)[..],
                            &unescape(&format!("{:?}", &tweet.text)).unwrap()[..],
                            &format!("{}",tweet.created_at.with_timezone(&Local))[..],
                            &format!("{}", &tweet.id)[..],
                        ),
                    )
                    .exec_console()
                    .exec_curl() {
                        CurlError => println!("slack Request error occured"),
                    };
                }
                Ok(())
            })
            .map_err(|e| println!("error: {}", e));

        rt::run(bot);
    }
}