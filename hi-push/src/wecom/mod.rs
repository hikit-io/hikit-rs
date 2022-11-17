#[cfg(feature = "wecom-model")]
mod model;

#[cfg(feature = "wecom-model")]
pub use model::*;

#[cfg(feature = "wecom")]
mod lib {
    use super::model::*;

    use std::sync::Arc;

    use derive_builder::Builder;
    use serde::{Deserialize, Serialize};
    use serde_repr::Serialize_repr;
    use tokio::sync::RwLock;

    pub struct Client {
        agentid: i64,
        cli: reqwest::Client,
        token_url: String,
        token: Arc<RwLock<Option<TokenResponse>>>,
    }

    impl Client {
        pub async fn new(
            client_id: &str,
            client_secret: &str,
            agentid: i64,
        ) -> Result<Client, crate::Error> {
            let cli = reqwest::Client::builder()
                .build()
                .map_err(|e| crate::InnerError::Http(e.to_string()))?;

            let token_url = format!(
                "https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}&corpsecret={}",
                client_id, client_secret
            );
            let client = Client {
                cli,
                agentid,
                token_url,
                token: Default::default(),
            };

            client.request_token().await?;

            Ok(client)
        }

        async fn request_token(&self) -> Result<TokenResponse, crate::Error> {
            let token = self
                .cli
                .get(&self.token_url)
                .send()
                .await
                .map_err(|e| crate::RetryError::Auth(e.to_string()))?
                .json::<TokenResponse>()
                .await
                .map_err(|e| crate::RetryError::Auth(e.to_string()))?;
            if token.errcode != 0 {
                return Err(crate::RetryError::Auth(token.errmsg).into());
            }
            self.set_token(token.clone()).await;
            Ok(token)
        }

        async fn set_token(&self, mut token: TokenResponse) {
            let expires_in = chrono::Utc::now().timestamp() as i64 + token.expires_in;
            token.expires_in = expires_in;
            *(self.token.write().await) = Some(token);
        }

        /*
            valid token before pushing
        */
        fn valid_token(&self, token: &TokenResponse) -> bool {
            if token.expires_in >= chrono::Utc::now().timestamp() as i64 {
                return false;
            }
            true
        }

        /*
            valid msg before pushing
        */

        pub fn agentid(&self) -> i64 {
            self.agentid
        }
    }

    #[async_trait::async_trait]
    impl<'b> crate::Pusher<'b, Message<'b>, Response> for Client {
        async fn push(&self, msg: &'b Message) -> Result<Response, crate::Error> {
            let token = self.token.clone();

            let token = token.read().await;

            let token = match token.clone() {
                Some(token) => token.clone(),
                None => match self.request_token().await {
                    Ok(token) => token,
                    //todo handle _e
                    Err(_e) => return Err(crate::RetryError::Auth("".to_string()).into()),
                },
            };

            if self.valid_token(&token) {
                return Err(crate::RetryError::Auth("token expired or invalid".to_string()).into());
            }

            let text = serde_json::to_string(msg).unwrap();

            let text = text.replace("\"agentid\":0", &format!("\"agentid\":{}", self.agentid));

            let resp = self
                .cli
                .post("https://qyapi.weixin.qq.com/cgi-bin/message/send")
                .query(&[("access_token", &token.access_token)])
                .header("Content-Type", "application/json;encode=utf-8")
                .body(text)
                .send()
                .await;

            match resp {
                Ok(resp) => match resp.error_for_status() {
                    Ok(resp) => Ok(resp.json::<Response>().await.unwrap()),
                    Err(e) => Err(crate::InnerError::Http(e.to_string()).into()),
                },
                Err(e) => Err(crate::InnerError::Http(e.to_string()).into()),
            }
        }
    }
}

#[cfg(feature = "wecom")]
pub use lib::*;

#[cfg(feature = "wecom")]
#[cfg_attr(feature = "wecom", test)]
mod tests {
    #[test]
    fn test_msg_builder() {
        use crate::*;
        use super::*;
        let msg = MessageBuilder::default()
            .to(To::Totag("()"))
            .agentid(123)
            .inner(InnerMesssage::Markdown { content: "()" })
            .build();
        assert_eq!(msg.is_ok(), true);

        let js = serde_json::to_string_pretty(&msg.unwrap()).unwrap();
        println!("{js}");

        let msg = MessageBuilder::default()
            .to(To::Totag("()"))
            .inner(InnerMesssage::Markdown { content: "()" })
            .build();
        assert_eq!(msg.is_err(), true);
    }

    #[tokio::test]
    async fn test_client() {
        use crate::*;
        use crate::Pusher;

        let client_id = std::env::var("WECOM_CLIENT_ID").unwrap();
        let client_secret = std::env::var("WECOM_CLIENT_SECRET").unwrap();
        let agent_id = std::env::var("WECOM_AGENT_ID")
            .unwrap()
            .parse::<i64>()
            .expect("");

        let cli = Client::new(&client_id, &client_secret, agent_id)
            .await
            .unwrap();

        let msg = MessageBuilder::default()
            .to(To::Touser("nieaowei".to_string()))
            .agentid(cli.agentid())
            .inner(InnerMesssage::Text {
                content: "hello harmongy",
            })
            .build()
            .unwrap();
        let resp = cli.push(&msg).await;
        println!("{resp:?}");

        assert_eq!(resp.is_ok(), true);
    }
}
