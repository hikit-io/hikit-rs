use async_trait::async_trait;
use serde::Serialize;

pub struct Client {
    cli: reqwest::Client,
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Serialize)]
pub struct Msg<'a> {
    pub destination: &'a str,
    pub enable_offline_messaging: Option<bool>, // under 1.5
    pub enable_historical_messaging: Option<bool>,
    pub payload: &'a str,
}

impl Client {
    const PUSH_URL: &'static str = "";
    pub fn new(client_id: &str, client_secret: &str) -> Result<Self, super::Error> {
        let cli = reqwest::Client::builder().build().unwrap();
        Ok(Self {
            cli,
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
        })
    }
}

#[async_trait]
impl<'b> super::Pusher<'b, Msg<'b>, ()> for Client {
    async fn push(&self, _msg: &'b Msg) -> Result<(), super::Error> {
        let _req = self
            .cli
            .post(Client::PUSH_URL)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send();

        Ok(())
    }
}
