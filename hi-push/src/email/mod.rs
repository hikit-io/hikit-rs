use std::time::Duration;

use async_trait::async_trait;
use lettre::{
    transport::smtp::authentication::{Credentials, Mechanism},
    SmtpTransport, Transport,
};

pub struct Message<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub to: Vec<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct PushResult<'a> {
    pub email: &'a str,
    pub success: bool,
    pub reason: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Response<'a> {
    results: Vec<PushResult<'a>>,
}

pub struct Client<'a> {
    client_id: &'a str,
    // client_secret: &'a str,
    // push_url: &'a str,
    cli: SmtpTransport,
}

impl<'a> Client<'a> {
    async fn new(client_id: &'a str, client_secret: &'a str, push_url: &'a str) -> Client<'a> {
        let creds = Credentials::new(client_id.to_string(), client_secret.to_string());
        let mailer = SmtpTransport::starttls_relay(push_url)
            .unwrap()
            .authentication(vec![Mechanism::Login])
            .port(587)
            .credentials(creds)
            .timeout(Duration::from_secs(10).into())
            .pool_config(Default::default())
            .build();

        Self {
            client_id,
            // client_secret,
            // push_url,
            cli: mailer,
        }
    }
}

#[async_trait]
impl<'b> super::Pusher<'b, Message<'b>, Response<'b>> for Client<'_> {
    async fn push(&self, msg: &'b Message) -> Result<Response<'b>, super::Error> {
        let mut results = Vec::new();

        for to in &msg.to {
            let to_mail = match format!("<{to}>").parse::<lettre::message::Mailbox>() {
                Ok(o) => o,
                Err(e) => {
                    results.push(PushResult {
                        email: to,
                        success: false,
                        reason: Some(e.to_string()),
                    });
                    continue;
                }
            };

            let msg = match lettre::Message::builder()
                .from(
                    format!("AgoraVideoCall <{}>", self.client_id)
                        .parse()
                        .unwrap(),
                )
                .to(to_mail)
                .body(msg.body.to_string())
            {
                Ok(o) => o,
                Err(e) => {
                    results.push(PushResult {
                        email: to,
                        success: false,
                        reason: Some(e.to_string()),
                    });
                    continue;
                }
            };

            match self.cli.send(&msg) {
                // @todo handle _resp
                Ok(_resp) => results.push(PushResult {
                    email: to,
                    success: true,
                    reason: None,
                }),
                Err(e) => results.push(PushResult {
                    email: to,
                    success: false,
                    reason: Some(e.to_string()),
                }),
            };
        }

        Ok(Response { results })
    }
}

#[cfg(test)]
mod tests {
    use crate::Pusher;

    #[tokio::test]
    async fn test_push() {
        let client_id = std::env::var("EMAIL_CLIENT_ID").unwrap();
        let client_secret = std::env::var("EMAIL_CLIENT_SECRET").unwrap();

        let cli = super::Client::new(&client_id, &client_secret, "smtp.office365.com").await;
        let msg = super::Message {
            title: "Hello Email",
            body: "This is test.",
            to: vec!["public@hikit.io", "1234"],
        };
        let res = cli.retry_push(&msg).await;
        println!("{:?}", res);
    }
}
