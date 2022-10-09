use std::{sync::Arc, time::Duration};

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use tokio::sync::RwLock;

pub struct Client<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    agentid: i64,
    cli: reqwest::Client,
    token_url: String,
    token: Arc<RwLock<Option<TokenResponse>>>,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub errcode: i32,
    pub errmsg: String,
    pub invaliduser: Option<String>,
    pub invalidparty: Option<String>,
    pub invalidtag: Option<String>,
    pub msgid: String,
    pub response_code: Option<String>,
}

impl<'a> Client<'a> {
    const MAX_TOKEN: i32 = 1000;

    pub async fn new(
        client_id: &'a str,
        client_secret: &'a str,
        agentid: i64,
    ) -> Result<Client<'a>, super::Error> {
        let cli = reqwest::Client::builder()
            .build()
            .map_err(|e| super::InnerError::Http(e.to_string()))?;

        let token_url = format!(
            "https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}&corpsecret={}",
            client_id, client_secret
        );
        let client = Client {
            client_id,
            cli,
            client_secret,
            agentid,
            token_url,
            token: Default::default(),
        };

        client.request_token().await?;

        Ok(client)
    }

    async fn request_token(&self) -> Result<TokenResponse, super::Error> {
        let token = self
            .cli
            .get(&self.token_url)
            .send()
            .await
            .map_err(|e| super::RetryError::Auth(e.to_string()))?
            .json::<TokenResponse>()
            .await
            .map_err(|e| super::RetryError::Auth(e.to_string()))?;
        if token.errcode != 0 {
            return Err(super::RetryError::Auth(token.errmsg).into());
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
impl<'b> super::Pusher<'b, Message<'b>, Response> for Client<'_> {
    async fn push(&self, msg: &'b Message) -> Result<Response, crate::Error> {
        let token = self.token.clone();

        let token = token.read().await;

        let token = match token.clone() {
            Some(token) => token.clone(),
            None => match self.request_token().await {
                Ok(token) => token,
                //todo handle _e
                Err(_e) => return Err(super::RetryError::Auth("".to_string()).into()),
            },
        };

        if self.valid_token(&token) {
            return Err(super::RetryError::Auth("token expired or invalid".to_string()).into());
        }

        let resp = self
            .cli
            .post("https://qyapi.weixin.qq.com/cgi-bin/message/send")
            .query(&[("access_token", &token.access_token)])
            .json(msg)
            .send()
            .await;

        match resp {
            Ok(resp) => match resp.error_for_status() {
                Ok(mut resp) => Ok(resp.json::<Response>().await.unwrap()),
                Err(e) => Err(super::InnerError::Http(e.to_string()).into()),
            },
            Err(e) => Err(super::InnerError::Http(e.to_string()).into()),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct TokenResponse {
    errcode: i32,
    errmsg: String,
    access_token: String,
    expires_in: i64,
}

#[derive(Debug, Serialize_repr, Clone)]
#[repr(u8)]
pub enum Bool {
    False = 0,
    True = 1,
}

impl From<Bool> for bool {
    fn from(b: Bool) -> Self {
        match b {
            Bool::False => false,
            Bool::True => true,
        }
    }
}

impl From<bool> for Bool {
    fn from(b: bool) -> Self {
        match b {
            true => Self::True,
            false => Self::False,
        }
    }
}

#[derive(Debug, Serialize, Builder)]
pub struct Message<'a> {
    #[serde(flatten)]
    pub to: To<'a>,
    pub msgtype: MsgType,
    #[builder(default)]
    pub agentid: i64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<Bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_duplicate_check: Option<Bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_check_interval: Option<Bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_id_trans: Option<Bool>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    #[builder(setter(custom), default)]
    pub inner: Option<InnerMesssage<'a>>,
}

impl<'a> MessageBuilder<'a> {
    pub fn inner(&mut self, value: InnerMesssage<'a>) -> &mut Self {
        match value {
            InnerMesssage::Text { .. } => self.msgtype = Some(MsgType::Text),
            InnerMesssage::Markdown { .. } => self.msgtype = Some(MsgType::Markdown),
            InnerMesssage::Textcard { .. } => self.msgtype = Some(MsgType::Textcard),
        }
        self.inner = Some(Some(value));
        self
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum To<'a> {
    Touser(&'a str),
    Toparty(&'a str),
    Totag(&'a str),
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum MsgType {
    #[default]
    Text,
    Markdown,
    Textcard,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum InnerMesssage<'a> {
    Text {
        content: &'a str,
    },
    Markdown {
        content: &'a str,
    },
    Textcard {
        title: &'a str,
        description: &'a str,
        url: &'a str,
        btntxt: Option<&'a str>,
    },
}

mod tests {
    use crate::Pusher;

    #[test]
    fn test_msg_builder() {
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
        use super::*;
        let cli = Client::new(
            "ww1792ac25c99153bd",
            "u-djCJvXF3_wGXhQeQnRSKBQ89Tr2VMWRFdUMwmr9Ds",
            1000024,
        )
            .await
            .unwrap();

        let msg = MessageBuilder::default()
            .to(To::Touser("nieaowei"))
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
