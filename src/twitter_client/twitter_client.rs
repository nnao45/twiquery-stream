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

    pub fn watch(self) -> bool {
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

        rt::run(bot);
        flag
    }
}