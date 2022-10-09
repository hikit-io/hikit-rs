use std::{default, ops::Add, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

use oauth2::TokenResponse;
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    reqwest::async_http_client,
    EmptyExtraTokenFields, RevocationErrorResponseType, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse,
};
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};
use tokio::sync::RwLock;

type Token = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

type AuthClient = oauth2::Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
>;

pub struct Client<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    auth: AuthClient,
    token: Arc<RwLock<Option<Token>>>,
    cli: reqwest::Client,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct Message<'a> {
    pub validate_only: Option<bool>,
    #[serde(borrow)]
    pub message: InnerMessage<'a>,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct InnerMessage<'a> {
    /*
       自定义消息负载，通知栏消息支持JSON格式字符串，透传消息支持普通字符串或者JSON格式字符串。样例："your data"，"{'param1':'value1','param2':'value2'}"。
       消息体中有message.data，没有message.notification和message.android.notification，消息类型为透传消息。
       如果用户发送的是网页应用的透传消息，那么接收消息中字段orignData为透传消息内容。
    */
    pub data: Option<&'a str>,
    pub notification: Option<Notification<'a>>,
    pub android: Option<AndroidConfig<'a>>,
    #[serde(borrow)]
    pub token: Vec<&'a str>, // max: 1000 除开token，4096Bytes
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct Notification<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub image: Option<&'a str>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Urgency {
    High,
    Normal,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct AndroidConfig<'a> {
    /*
    0：对每个应用发送到该用户设备的离线消息只会缓存最新的一条。
    -1：对所有离线消息都缓存。默认值是-1
    1~100：离线消息缓存分组标识
    */
    pub collapse_key: Option<i64>,
    pub urgency: Option<Urgency>,
    pub category: Option<&'a str>,
    //消息缓存时间，单位是秒 例如： 1000s 。 默认值为“86400s”（1天），最大值为“1296000s”（15天）。
    pub ttl: Option<&'a str>,
    // 批量任务消息标识，消息回执时会返回给应用服务器，应用服务器可以识别bi_tag对消息的下发情况进行统计分析。
    pub bi_tag: Option<&'a str>,
    pub receipt_id: Option<&'a str>,
    /*
       快应用发送透传消息时，指定小程序的模式类型，小程序有两种模式开发态和生产态，取值如下：
       1：开发态
       2：生产态
       默认值是2。
    */
    pub fast_app_target: i64,
    pub data: Option<&'a str>,
    pub notification: Option<AndroidNotification<'a>>,
}

#[derive(Debug, Serialize_repr, Clone)]
#[repr(u8)]
pub enum Style {
    Default = 0,
    BigText = 1,
    Inbox = 2,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Importance {
    Low,
    Normal,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct AndroidNotification<'a> {
    pub image: Option<&'a str>,
    pub icon: Option<&'a str>,
    pub color: Option<&'a str>,
    pub sound: Option<&'a str>,
    pub default_sound: Option<bool>,
    pub tag: Option<&'a str>,
    pub importance: Option<Importance>,
    pub click_action: ClickAction<'a>,
    pub body_loc_key: Option<&'a str>,
    pub body_loc_args: &'a [&'a str],
    pub title_loc_key: Option<&'a str>,
    pub title_loc_args: &'a [&'a str],
    pub channel_id: Option<&'a str>,
    pub notify_summary: Option<&'a str>,
    pub style: Option<Style>,
    pub big_title: Option<&'a str>,
    pub big_body: Option<&'a str>,
    pub notify_id: Option<i64>,
    pub group: Option<&'a str>,
    pub badge: Option<Badge<'a>>,
    pub foreground_show: Option<bool>,
    pub ticker: Option<&'a str>,
    pub when: Option<&'a str>,
    pub local_only: bool,
    pub use_default_vibrate: bool,
    pub use_default_light: bool,
    pub visibility: Option<&'a str>,
    pub vibrate_config: Vec<&'a str>,
    pub light_settings: LightSettings<'a>,
    pub auto_clear: Option<i8>,
}

#[derive(Debug, Serialize_repr, Default, Clone)]
#[repr(u8)]
pub enum ClickActionType {
    Intent = 1,
    Web = 2,
    #[default]
    Main = 3,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "80000000")]
    Success,
    #[serde(rename = "80100000")]
    PartFailedErr,
    #[serde(rename = "80100001")]
    ParameterError,
    #[serde(rename = "80100002")]
    TokenMustOne,
    #[serde(rename = "80100003")]
    MsgError,
    #[serde(rename = "80100004")]
    TTLErr,
    #[serde(rename = "80200001")]
    TokenFailedErr,
    #[serde(rename = "80200003")]
    TokenTimeoutErr,
    #[serde(rename = "80300007")]
    TokenInvalid,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct ClickAction<'a> {
    #[serde(rename = "type")]
    pub type_field: ClickActionType,
    pub intent: Option<&'a str>,
    pub url: Option<&'a str>,
    pub action: Option<&'a str>,
}

impl<'a> ClickAction<'a> {
    pub fn new_intent(action: &'a str) -> Self {
        Self {
            type_field: ClickActionType::Intent,
            action: Some(action),
            ..Default::default()
        }
    }
    pub fn new_web(url: &'a str) -> Self {
        Self {
            type_field: ClickActionType::Web,
            url: Some(url),
            ..Default::default()
        }
    }
    pub fn new_main() -> Self {
        Self {
            type_field: ClickActionType::Main,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct Badge<'a> {
    pub add_num: Option<i64>,
    pub class: &'a str,
    pub set_num: Option<i64>,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct LightSettings<'a> {
    pub color: Color,
    pub light_on_duration: &'a str,
    pub light_off_duration: &'a str,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct Color {
    pub alpha: Option<i64>,
    pub red: Option<i64>,
    pub green: Option<i64>,
    pub blue: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub msg: String,
    pub code: Code,
    pub request_id: String,
}

impl<'a> Client<'a> {
    const TOKEN_URL: &'static str = "https://oauth-login.cloud.huawei.com/oauth2/v3/token";
    const PUSH_URL: &'static str = "https://oauth-login.cloud.huawei.com/oauth2/v3/token";

    pub async fn new(
        client_id: &'a str,
        client_secret: &'a str,
    ) -> Result<Client<'a>, super::Error> {
        let auth = BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::new(Self::TOKEN_URL.to_string())
                .map_err(|e| super::RetryError::Auth(e.to_string()))?,
            Some(
                TokenUrl::new(Self::TOKEN_URL.to_string())
                    .map_err(|e| super::RetryError::Auth(e.to_string()))?,
            ),
        );

        let auth = auth.set_auth_type(oauth2::AuthType::RequestBody);

        let cli = reqwest::Client::builder()
            .build()
            .map_err(|e| super::InnerError::Http(e.to_string()))?;

        let res = Client::<'a> {
            auth,
            client_id,
            client_secret,
            token: Default::default(),
            cli,
        };

        res.request_token().await?;

        Ok(res)
    }

    /*
        request and set token
    */
    async fn request_token(&self) -> Result<Token, super::Error> {
        let token = self
            .auth
            .exchange_client_credentials()
            .request_async(async_http_client)
            .await
            .map_err(|e| super::RetryError::Auth(e.to_string()))?;
        self.set_token(token.clone()).await;

        Ok(token)
    }

    async fn set_token(&self, mut token: Token) {
        let expires_in =
            chrono::Utc::now().timestamp() as u64 + (token.expires_in().unwrap().as_secs());
        token.set_expires_in(Some(&Duration::from_secs(expires_in)));
        *(self.token.write().await) = Some(token);
    }

    /*
        valid token before pushing
    */
    fn valid_token(&self, token: &Token) -> bool {
        let expires = token.expires_in();
        if expires.is_none() {
            return false;
        }
        if expires.unwrap().as_secs() <= chrono::Utc::now().timestamp() as u64 {
            return false;
        }
        true
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
            .post(Self::PUSH_URL)
            .bearer_auth(token.access_token().secret())
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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_push() {
        use super::*;

        let _cli = Client::new(
            "101383969",
            "96ae46309c1a891bec33ae073b8fec7c1b90cdaa6b1c575150452497706b2d4f",
        )
            .await
            .unwrap();
    }
}
