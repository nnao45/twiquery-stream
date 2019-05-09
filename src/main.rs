mod twitter_client;

fn main() {
    let tc = twitter_client::TwitterClient::new().unwrap();
    tc.watch();
}