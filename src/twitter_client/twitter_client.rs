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

pub const RESET_FLAG: bool = true;
pub const UNRESET_FLAG: bool = false;

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

    pub fn watch<T: Streamer>(self, streamer: T) -> bool {
        let consumer_key: &str = &self.config.consumer_key.replace("\n", "");
        let consumer_secret: &str = &self.config.consumer_secret.replace("\n", "");
        let access_token: &str = &self.config.access_token.replace("\n", "");
        let access_token_secret: &str = &self.config.access_token_secret.replace("\n", "");
        let track: &str = &self.config.track;
        let mut lang: &str = &self.config.filter_lang;
        if lang == "none" {
            lang = "";
        };
        if self.config.is_debug {
            info!("track: {}", track);
            info!("lanuage: {}", lang);
        };
        let mut flag = UNRESET_FLAG;
        let bot = TwitterStreamBuilder::filter(twitter_stream::Token::new(
                    consumer_key,
                    consumer_secret,
                    access_token,
                    access_token_secret,
                ))
            .language(lang)
            .track(Some(track))
            .listen()
            .unwrap()
            .flatten_stream()
            .for_each(move |json| {
                if let Ok(StreamMessage::Tweet(tweet)) = StreamMessage::from_str(&json) {
                    Executer::new(
                        &self.config.slack_url,
                        self.config.post_slack_enabled,
                        TweiqueryData::new(
                            &self.config.track,
                            &format!("{}", &tweet.user.name)[..],
                            &format!("{}", &tweet.user.screen_name)[..],
                            &unescape(&format!("{:?}", &tweet.text)).unwrap()[..],
                            &format!("{}",tweet.created_at.with_timezone(&Local))[..],
                            &format!("{}", &tweet.id)[..],
                        ),
                    )
                    .exec();
                } else if let Err(e) = StreamMessage::from_str(&json)  {
                    error!("error is {:?}, json is {}", e, &json);
                }
                Ok(())
            })
            .map_err(move |e| {
                let msg: &str = &format!("{}", e);
                error!("{}", msg);
                match msg {
                    "420 <unknown status code>" => flag = UNRESET_FLAG,
                    _ => flag = RESET_FLAG,
                }
            });

        info!("Start watch steram api");
        streamer.stream_run(bot);
        error!("Stop watch steram api");
        flag
    }
}

pub trait Streamer {
    fn new() -> Self;
    fn stream_run<F>(self, future: F)
    where F: Future<Item = (), Error = ()> + Send + 'static;
}

pub struct TwitterStreamer {
}

impl Streamer for TwitterStreamer {
    fn new() -> Self {
        TwitterStreamer{
        }
    }
    fn stream_run<F>(self, future: F)
    where F: Future<Item = (), Error = ()> + Send + 'static,
    {
        rt::run(future);
    }
}

#[cfg(test)]
mod twitter_client_tests {
    use std::env;
    use futures::Future;
    use super::{TwitterClient, Streamer,  Config, UNRESET_FLAG};
    #[test]
    fn config_load_test() {
        let dummy_consumer_key = "dummy_consumer_key";
        let dummy_consumer_secret = "dummy_consumer_key";
        let dummy_access_token = "dummy_access_token";
        let dummy_access_token_secret = "dummy_access_token_secret";
        let dummy_track = "dummy_track";
        let dummy_slack_url = "https://dummy.slack.com";
        let dummy_is_debug = true;
        let dummy_slack_enabled = true;
        let dummy_filter_lang = "ja";
        env::set_var("CONSUMER_KEY", dummy_consumer_key);
        env::set_var("CONSUMER_SECRET",dummy_consumer_secret);
        env::set_var("ACCESS_TOKEN", dummy_access_token);
        env::set_var("ACCESS_TOKEN_SECRET", dummy_access_token_secret);
        env::set_var("TRACK", dummy_track);
        env::set_var("SLACK_URL", dummy_slack_url);
        env::set_var("IS_DEBUG", format!("{}", dummy_is_debug));
        env::set_var("POST_SLACK_ENABLED", format!("{}", dummy_slack_enabled));
        env::set_var("FILTER_LANG", dummy_filter_lang);
        match Config::new() {
            Ok(config) => {
                assert_eq!(dummy_consumer_key, config.consumer_key);
                assert_eq!(dummy_consumer_secret, config.consumer_secret);
                assert_eq!(dummy_access_token, config.access_token);
                assert_eq!(dummy_access_token_secret, config.access_token_secret);
                assert_eq!(dummy_track, config.track);
                assert_eq!(dummy_slack_url, config.slack_url);
                assert_eq!(dummy_is_debug, config.is_debug);
                assert_eq!(dummy_slack_enabled, config.post_slack_enabled);
                assert_eq!(dummy_filter_lang, config.filter_lang);
            },
            _ => panic!(),
        }
    }

    pub struct TestStreamer {
    }

    impl Streamer for TestStreamer {
        fn new() -> Self {
            TestStreamer{
            }
        }
        fn stream_run<F>(self, _future: F)
        where F: Future<Item = (), Error = ()> + Send + 'static,
        {
            ();
        }
    }

    #[test]
    fn watch_test() {
        let dummy_consumer_key = "dummy_consumer_key";
        let dummy_consumer_secret = "dummy_consumer_key";
        let dummy_access_token = "dummy_access_token";
        let dummy_access_token_secret = "dummy_access_token_secret";
        let dummy_track = "dummy_track";
        let dummy_slack_url = "https://dummy.slack.com";
        let dummy_is_debug = true;
        let dummy_slack_enabled = true;
        let dummy_filter_lang = "ja";
        env::set_var("CONSUMER_KEY", dummy_consumer_key);
        env::set_var("CONSUMER_SECRET",dummy_consumer_secret);
        env::set_var("ACCESS_TOKEN", dummy_access_token);
        env::set_var("ACCESS_TOKEN_SECRET", dummy_access_token_secret);
        env::set_var("TRACK", dummy_track);
        env::set_var("SLACK_URL", dummy_slack_url);
        env::set_var("IS_DEBUG", format!("{}", dummy_is_debug));
        env::set_var("POST_SLACK_ENABLED", format!("{}", dummy_slack_enabled));
        env::set_var("FILTER_LANG", dummy_filter_lang);
        match Config::new() {
            Ok(config) => {
                assert_eq!(dummy_consumer_key, config.consumer_key);
                assert_eq!(dummy_consumer_secret, config.consumer_secret);
                assert_eq!(dummy_access_token, config.access_token);
                assert_eq!(dummy_access_token_secret, config.access_token_secret);
                assert_eq!(dummy_track, config.track);
                assert_eq!(dummy_slack_url, &config.slack_url);
                assert_eq!(dummy_is_debug, config.is_debug);
                assert_eq!(dummy_slack_enabled, config.post_slack_enabled);
                assert_eq!(dummy_filter_lang, config.filter_lang);
                assert_eq!(TwitterClient::new(&config).watch(TestStreamer::new()), UNRESET_FLAG);
            },
            _ => panic!(),
        }
    }
}