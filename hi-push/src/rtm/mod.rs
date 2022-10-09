pub struct Client {
    cli: reqwest::Client,
}

impl Client {
    pub fn new() -> Result<Self, super::Error> {
        let cli = reqwest::Client::builder().build().unwrap();
        Ok(Self { cli })
    }
}
