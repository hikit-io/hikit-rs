use async_trait::async_trait;
use serde::Serialize;

pub struct Client {
    cli: reqwest::Client,
    client_id: String,
    client_secret: String,
}

#[derive(Serialize)]
pub struct Msg<'a> {
    destination: &'a str,
    enable_offline_messaging: bool,
    enable_historical_messaging: bool,
    payload: &'a str,
}

impl Client {
    pub fn new(client_id: &str, client_secret: &str) -> Result<Self, super::Error> {
        let cli = reqwest::Client::builder()
            .build()
            .unwrap();
        Ok(Self {
            cli,
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
        })
    }
}

#[async_trait]
impl<'b> super::Pusher<'b, Msg<'b>, ()> for Client {
    async fn push(&self, msg: &'b Msg) -> Result<(), super::Error> {

        let req = self.cli.post("/").basic_auth(&self.client_id, Some(&self.client_secret));

        Ok(())
    }
}
