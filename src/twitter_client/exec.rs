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
                        title: format!(":twitter: Tweet"),
                        value: format!("```{}```\nhttps://twitter.com/statuses/{}", tweet, id),
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
        info!("{} [{}]\n{}", data.title, data.footer, &data.fields[0].value);
        if self.post_slack_enabled {
            for _ in 1..5 {
                match self.exec_curl() {
                    Ok(()) => {
                        info!("Slack request done");
                        break
                        },
                    _ => {
                        error!("Slack request may error occured");
                    },
                }
                std::thread::sleep(Duration::from_secs(5));
            }
        }
    }

    pub fn exec_curl(&self) -> Result<(), CurlError>{
        let row = &self.data;
        let row_str = serde_json::to_string(&row).unwrap_or("{\"text\": \"error occured\"}".to_string());
        let mut bytes = row_str.as_bytes();
        let mut easy = Easy::new();
        easy.url(&self.slack_url.replace("\n", ""))?;
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

#[cfg(test)]
mod exec_tests {
    use super::{Executer, TweiqueryData, Server};

    #[test]
    fn exec_test() {
        let dummy_slack_enabled = true;
        let dummy_data = TweiqueryData::new("dummy_track", "dummy", "@dummy", "Dummy Hello World!!", "1970-01-01 00:00:00 +00:00", "999999999999999");
        let s = Server::new();
        s.receive(
            "\
             POST / HTTP/1.1\r\n\
             Host: 127.0.0.1:$PORT\r\n\
             Accept: */*\r\n",
        );
        s.send("HTTP/1.1 200 OK\r\n\r\n");
        let e = Executer::new(&s.url("/"), dummy_slack_enabled, dummy_data);
        e.exec()
    }
}