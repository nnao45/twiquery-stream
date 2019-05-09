mod twitter_client;

fn main() {
    twitter_client::TwitterClient::new().unwrap().watch();
}