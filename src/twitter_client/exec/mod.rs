extern crate curl;
extern crate chrono;

use std::io::{Read};

use curl::easy::{Easy, List};
use curl::Error as CurlError;

use serde::{Serialize, Deserialize};

use slog::{slog_info,slog_error};

#[derive(Serialize, Deserialize, Debug)]
pub struct TweiqueryData {
    attachments : Vec<TweiqueryDataAttachments>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TweiqueryDataAttachments {
    title: String,
    pretext: String,
    color: String,
    fields: Vec<TweiqueryDataAttachmentsFields>,
    footer: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TweiqueryDataAttachmentsFields {
    title: String,
    value: String,
}

impl TweiqueryData {
    pub fn new(track: &str, user_name: &str, screen_name: &str, tweet: &str, date: &str, id :&str) -> Self {
        TweiqueryData {
            attachments : vec![TweiqueryDataAttachments{
                title: format!("{} @{}", user_name, screen_name),
                pretext: format!("🌟{}の関連ツイートを取得しました", track),
                color: "#27aeff".to_string(),
                fields: vec![
                    TweiqueryDataAttachmentsFields {
                        title: format!(":twitter: https://twitter.com/statuses/{}", id),
                        value: format!("```{}```", tweet),
                    },
                ],
                footer: format!("{}", date),
            }]
        }
    }
}

#[derive(Debug)]
pub struct Executer {
    slack_url: String,
    post_slack_enabled: bool,
    pub data: TweiqueryData,
}

impl Executer {
    pub fn new(u: &str, p: bool, d: TweiqueryData) -> Self {
        Executer {
            slack_url: u.to_string(),
            post_slack_enabled: p,
            data: d,
        }
    }

    pub fn exec(self) {
        let data = &self.data.attachments[0];
        slog_info!(slog_scope::logger(), "{} [{}]\n{}", data.title, data.footer, &data.fields[0].value);
        if self.post_slack_enabled {
            match self.exec_curl() {
                Ok(()) => slog_info!(slog_scope::logger(), "Slack request done"),
                _ => slog_error!(slog_scope::logger(), "Slack request may error occured"),
            };
        }
    }

    fn exec_curl(self) -> Result<(), CurlError> {
        let row = self.data;
        let row_str = serde_json::to_string(&row).unwrap_or("{\"text\": \"error occured\"}".to_string());
        let mut bytes = row_str.as_bytes();
        let mut easy = Easy::new();
        easy.url(&self.slack_url)?;
        let mut list = List::new();
        list.append("Content-type: application/json")?;
        easy.http_headers(list)?;

        easy.post(true)?;
        easy.post_field_size(bytes.len() as u64)?;

        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            Ok(bytes.read(buf).unwrap_or(0))
        })?;

        transfer.perform()
    }
}