use reqwest::StatusCode;
use std::collections::HashMap;
use thiserror::Error;

use async_trait::async_trait;
use flatten_json_object::{ArrayFormatting, Flattener};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tokio::sync::RwLock;

#[cfg(all(feature = "mysql", feature = "mongo"))]
compile_error!("mysql and mongo not enable both.");

#[cfg(feature = "apns")]
pub mod apns;
#[cfg(feature = "email")]
pub mod email;
#[cfg(feature = "fcm")]
pub mod fcm;
#[cfg(feature = "huawei")]
pub mod huawei;
#[cfg(feature = "rtm")]
pub mod rtm;
#[cfg(feature = "wecom")]
pub mod wecom;
#[cfg(feature = "xiaomi")]
pub mod xiaomi;

#[cfg(feature = "grpc")]
pub mod grpc;

#[cfg(feature = "http")]
pub mod http;

#[cfg(all(any(feature = "http", feature = "grpc")))]
pub mod service;

pub enum Transparent<'a> {
    Text(&'a str),
    Json(serde_json::Map<String, serde_json::Value>),
}

#[derive(Debug, Clone)]
pub enum Body<'a> {
    Transparent(&'a str),
    Notify { title: &'a str, body: &'a str },
}

#[derive(Debug, Clone)]
pub struct ApnsExtra<'a> {
    pub topic: &'a str,
    pub push_type: apns::ApnsPushType,
}

#[derive(Debug, Clone)]
pub enum WecomExtra<'a> {
    Markdown(bool),
    Text { url: &'a str, btntxt: &'a str },
}

#[derive(Debug, Default)]
pub struct PushResult {
    pub request_id: String,
    pub code: String,
    pub reason: String,
    pub success: i64,
    pub failure: i64,
    pub invalid_tokens: Vec<String>,
}

#[derive(Debug)]
pub struct PushResults {
    pub success: i64,
    pub failure: i64,
    pub results: Vec<PushResult>,
}

#[derive(Debug, Clone)]
pub struct Message<'a> {
    pub tokens: &'a [&'a str],
    pub body: Body<'a>,
    pub android: Option<AndroidExtra<'a>>,
    pub apns: Option<ApnsExtra<'a>>,
    pub wecom: Option<WecomExtra<'a>>,
}

#[derive(Debug, PartialEq, Error)]
#[error(transparent)]
pub struct Error {
    inner: InnerError,
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum InnerError {
    #[error("new client error: `{0}`")]
    Client(String),
    #[error("http error: `{0}`")]
    Http(String),
    #[error("format error: `{0}`")]
    Format(String),
    #[error("response error: `{0}`")]
    Response(String),
    #[error("biz error: `{0}`")]
    Biz(String),
    #[error("retryable error : `{0}`")]
    RetryError(RetryError),
    #[error("missing required paramater: `{0}`")]
    MissingRequired(String),
    #[error("no exist client")]
    NoExistClient,
    #[error("serde error: `{0}`")]
    Serde(String),
    #[error("message limit")]
    MessageLimit,
    #[error("unknown error: `{0}`")]
    Unknown(String),
}
//
// impl From<reqwest::Error> for Error {
//     fn from(e: reqwest::Error) -> Self {
//         let inner = if e.is_builder() {
//             InnerError::Client(e.to_string())
//         } else if e.is_status() {
//             match e.status().unwrap() {
//                 StatusCode::BAD_REQUEST => InnerError::MissingRequired(e.to_string()),
//                 StatusCode::SERVICE_UNAVAILABLE => {
//                     InnerError::RetryError(RetryError::Server(e.to_string()))
//                 }
//                 StatusCode::GATEWAY_TIMEOUT => {
//                     InnerError::RetryError(RetryError::Timeout(e.to_string()))
//                 }
//                 StatusCode::REQUEST_TIMEOUT => {
//                     InnerError::RetryError(RetryError::Timeout(e.to_string()))
//                 }
//                 StatusCode::UNAUTHORIZED => InnerError::RetryError(RetryError::Auth(e.to_string())),
//                 _ => InnerError::Unknown(e.to_string()),
//             }
//         } else if e.is_timeout() {
//             InnerError::RetryError(RetryError::Timeout(e.to_string()))
//         } else if e.is_body() {
//             InnerError::Format(e.to_string())
//         } else if e.is_decode() {
//             InnerError::Serde(e.to_string())
//         } else {
//             InnerError::Unknown("unknown".to_string())
//         };
//         Self { inner }
//     }
// }

#[derive(Debug, Eq, PartialEq, Error)]
pub enum RetryError {
    #[error("auth error: `{0}`")]
    Auth(String),
    #[error("server error: `{0}`")]
    Server(String),
    #[error("timeout error: `{0}`")]
    Timeout(String),
    #[error("qps limiter")]
    QPS,
}

impl From<RetryError> for Error {
    fn from(e: RetryError) -> Self {
        Self {
            inner: InnerError::RetryError(e),
        }
    }
}

impl From<InnerError> for Error {
    fn from(e: InnerError) -> Self {
        Self { inner: e }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Http,
    Format,
    Auth,
    Response,
    Biz,
}

// Pusher
// Message pushing abstract.
// limit:
// - max token number
// - retryable
// - qps
// - body
// - msg field
#[async_trait]
pub trait Pusher<'b, M, R>
    where
        M: Sync,
        R: Send,
{
    const TOKEN_LIMIT: usize = 500;

    async fn push(&self, msg: &'b M) -> Result<R, Error>;

    async fn retry_push(&self, msg: &'b M) -> Result<R, Error> {
        tokio_retry::RetryIf::spawn(
            tokio_retry::strategy::ExponentialBackoff::from_millis(2)
                .factor(1000)
                .take(3),
            || self.push(msg),
            |e: &Error| match e.inner {
                InnerError::RetryError(_) => true,
                _ => false,
            },
        )
            .await
    }

    async fn retry_batch_push(&self, msgs: &'b [&'b M]) -> Result<Vec<R>, Error> {
        let mut resps = Vec::new();
        for msg in msgs {
            let resp = self.retry_push(msg).await?;
            resps.push(resp);
        }
        Ok(resps)
    }
}

pub trait FromMessage<'a>: TryFrom<Message<'a>> {}

impl<'a> TryFrom<Message<'a>> for xiaomi::Message<'a> {
    type Error = Error;

    fn try_from(msg: Message<'a>) -> Result<Self, Self::Error> {
        let extra = msg.android.as_ref();
        Ok(match msg.body {
            Body::Transparent(data) => xiaomi::Message {
                payload: Some(data),
                pass_through: xiaomi::Passtrough::Transparent,
                ..Default::default()
            },
            Body::Notify { title, body, .. } => xiaomi::Message {
                title,
                description: body,
                registration_id: Some(msg.tokens.join("|")),
                pass_through: xiaomi::Passtrough::Notice,
                restricted_package_name: extra.map_or("", |v| v.package_name),
                extra: xiaomi::Extra {
                    notify_foreground: extra.map_or(None, |v| v.foreground_show.map(|v| v.into())),
                    notify_effect: extra.map_or(None, |v| {
                        if !v.click_action.is_empty() {
                            Some(xiaomi::NotifyEffect::Intent)
                        } else {
                            None
                        }
                    }),
                    intent_uri: extra.map_or(None, |v| {
                        if !v.click_action.is_empty() {
                            Some(v.click_action)
                        } else {
                            None
                        }
                    }),
                    job_key: extra.map_or(None, |v| {
                        if !v.click_action.is_empty() {
                            Some(v.click_action)
                        } else {
                            None
                        }
                    }),
                    ..Default::default()
                }
                    .into(),
                ..Default::default()
            },
        })
    }
}

impl<'a> FromMessage<'a> for xiaomi::Message<'a> {}

impl<'a> FromMessage<'a> for Vec<xiaomi::Message<'a>> {}

impl<'a> TryFrom<Message<'a>> for Vec<xiaomi::Message<'a>> {
    type Error = Error;

    fn try_from(msg: Message<'a>) -> Result<Self, Self::Error> {
        let tokens = msg.tokens;

        let mi_msg: xiaomi::Message = msg.try_into()?;

        let mut msgs = Vec::new();
        for i in 0..(tokens.len() / 1000) {
            let mut mi_tokens = Vec::new();

            let mut msg = mi_msg.clone();
            let start_index = i * 1000;
            let end_index = if tokens.len() - start_index >= 1000 {
                1000
            } else {
                tokens.len()
            };
            for j in start_index..end_index {
                mi_tokens.push(tokens[i * 1000 + j]);
            }
            msg.registration_id = Some(mi_tokens.join("|"));
            msgs.push(msg);
        }

        Ok(msgs)
    }
}

impl<'a> FromMessage<'a> for huawei::Message<'a> {}

impl<'a> TryFrom<Message<'a>> for huawei::Message<'a> {
    type Error = Error;

    fn try_from(msg: Message<'a>) -> Result<Self, Self::Error> {
        let extra = msg.android.as_ref();

        match msg.body {
            Body::Transparent(data) => Ok(huawei::Message {
                validate_only: Some(false),
                message: huawei::InnerMessage {
                    data: Some(data),
                    android: Some(huawei::AndroidConfig {
                        ..Default::default()
                    }),
                    token: msg.tokens.to_vec(),
                    ..Default::default()
                },
            }),
            Body::Notify { title, body } => Ok(huawei::Message {
                validate_only: Some(false),
                message: huawei::InnerMessage {
                    token: msg.tokens.to_vec(),
                    notification: Some(huawei::Notification {
                        title,
                        body,
                        ..Default::default()
                    }),
                    android: Some(huawei::AndroidConfig {
                        collapse_key: extra.map_or(None, |e| e.collapse_key),
                        urgency: extra.map_or(None, |e| {
                            e.priority.as_ref().map_or(None, |e| match e {
                                Priority::High => Some(huawei::Urgency::High),
                                Priority::Normal => Some(huawei::Urgency::Normal),
                            })
                        }),
                        // ttl: extra.map_or(None, |e| e.ttl.map_or(None, |e| Some(e))),
                        notification: Some(huawei::AndroidNotification {
                            sound: extra.map_or(None, |e| e.sound),
                            icon: extra.map_or(None, |e| e.icon),
                            image: extra.map_or(None, |e| Some(e.image)),
                            tag: extra.map_or(None, |e| e.tag),
                            body_loc_key: extra.map_or(None, |e| Some(e.body_loc_key)),
                            body_loc_args: extra.map_or(Default::default(), |e| e.body_loc_args),
                            title_loc_key: extra
                                .map_or(Default::default(), |e| Some(e.title_loc_key)),
                            title_loc_args: extra.map_or(Default::default(), |e| e.title_loc_args),
                            channel_id: extra.map_or(None, |e| Some(e.channel_id)),
                            ticker: extra.map_or(None, |e| Some(e.ticker)),
                            click_action: extra
                                .map(|e| huawei::ClickAction::new_intent(e.click_action))
                                .ok_or(InnerError::Format("missing click action".to_string()))?,
                            visibility: extra.map_or(None, |e| match e.visibility {
                                Visibility::Unspecified => Some(""),
                                Visibility::Private => Some("PRIVATE"),
                                Visibility::Public => Some("PUBLIC"),
                                Visibility::Secret => Some("SECRET"),
                            }),
                            auto_clear: extra.map_or(None, |e| e.auto_clear),
                            foreground_show: extra.map_or(None, |e| e.foreground_show),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    data: None,
                },
            }),
        }
    }
}


impl<'a> FromMessage<'a> for Vec<huawei::Message<'a>> {}

impl<'a> TryFrom<Message<'a>> for Vec<huawei::Message<'a>> {
    type Error = Error;

    fn try_from(msg: Message<'a>) -> Result<Self, Self::Error> {
        let tokens = msg.tokens;

        let hw_msg: huawei::Message = msg.try_into()?;

        let mut msgs = Vec::new();

        for i in 0..(tokens.len() / 1000) + 1 {
            let mut hw_tokens = Vec::new();

            let mut msg = hw_msg.clone();

            let start_index = i * 1000;
            let end_index = if tokens.len() - start_index >= 1000 {
                1000
            } else {
                tokens.len()
            };
            for j in start_index..end_index {
                hw_tokens.push(tokens[j]);
            }
            msg.message.token = hw_tokens;
            msgs.push(msg);
        }

        Ok(msgs)
    }
}

impl<'a> FromMessage<'a> for fcm::Message {}

impl<'a> TryFrom<Message<'a>> for fcm::Message {
    type Error = Error;

    fn try_from(msg: Message<'a>) -> Result<Self, Self::Error> {
        let extra = msg.android.as_ref();

        match msg.body {
            Body::Transparent(data) => {
                let data = if let Ok(val) = serde_json::from_str::<serde_json::Value>(data) {
                    let val = Flattener::new()
                        .set_key_separator(".")
                        .set_array_formatting(ArrayFormatting::Plain)
                        .set_preserve_empty_arrays(false)
                        .set_preserve_empty_objects(false)
                        .flatten(&val)
                        .unwrap();
                    serde_json::from_value::<HashMap<String, String>>(val)
                        .map_err(|e| InnerError::Serde(e.to_string()))?
                } else {
                    HashMap::from_iter([("text".to_string(), data.to_string())])
                };
                Ok(fcm::Message {
                    data: Some(data),
                    token: Some(msg.tokens.join("|")),
                    ..Default::default()
                })
            }
            Body::Notify { title, body } => Ok(fcm::Message {
                notification: Some(fcm::Notification {
                    body: Some(body.to_string()),
                    title: Some(title.to_string()),
                    ..Default::default()
                }),
                android: Some(fcm::AndroidConfig {
                    collapse_key: extra.map_or(None, |e| {
                        e.collapse_key.map_or(None, |e| Some(e.to_string()))
                    }),
                    priority: extra.map_or(None, |e| {
                        e.priority.as_ref().map_or(None, |e| match e {
                            Priority::High => Some("high".to_string()),
                            Priority::Normal => Some("normal".to_string()),
                        })
                    }),
                    ttl: extra.map_or(None, |e| e.ttl.map_or(None, |e| Some(format!("{}s", e)))),
                    notification: Some(fcm::AndroidNotification {
                        sound: extra
                            .map_or(None, |e| e.sound.map_or(None, |e| Some(e.to_string()))),
                        icon: extra.map_or(None, |e| e.icon.map_or(None, |e| Some(e.to_string()))),
                        image: extra.map_or(None, |e| Some(e.image.to_string())),
                        tag: extra.map_or(None, |e| e.tag.map_or(None, |e| Some(e.to_string()))),
                        body_loc_key: extra.map_or(None, |e| Some(e.body_loc_key.to_string())),
                        body_loc_args: extra.map_or(None, |e| {
                            Some(e.body_loc_args.iter().map(|e| e.to_string()).collect::<_>())
                        }),
                        title_loc_key: extra.map_or(None, |e| Some(e.title_loc_key.to_string())),
                        title_loc_args: extra.map_or(None, |e| {
                            Some(
                                e.title_loc_args
                                    .iter()
                                    .map(|e| e.to_string())
                                    .collect::<_>(),
                            )
                        }),
                        channel_id: extra.map_or(None, |e| Some(e.channel_id.to_string())),
                        ticker: extra.map_or(None, |e| Some(e.ticker.to_string())),
                        click_action: extra.map_or(None, |e| Some(e.click_action.to_string())),
                        visibility: extra.map_or(None, |e| match e.visibility {
                            Visibility::Unspecified => Some("".to_string()),
                            Visibility::Private => Some("PRIVATE".to_string()),
                            Visibility::Public => Some("PUBLIC".to_string()),
                            Visibility::Secret => Some("SECRET".to_string()),
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        }
    }
}


impl<'a> FromMessage<'a> for Vec<fcm::Message> {}

impl<'a> TryFrom<Message<'a>> for Vec<fcm::Message> {
    type Error = Error;

    fn try_from(msg: Message<'a>) -> Result<Self, Self::Error> {
        let tokens = msg.tokens;

        let hw_msg: fcm::Message = msg.try_into()?;

        let mut msgs = Vec::new();

        for i in 0..(tokens.len() / 1000) + 1 {
            let mut fcm_tokens = String::new();

            let mut msg = hw_msg.clone();

            let start_index = i * 1000;
            let end_index = if tokens.len() - start_index >= 1000 {
                1000
            } else {
                tokens.len()
            };
            for j in start_index..end_index {
                fcm_tokens += tokens[j];
                if j != end_index - 1 {
                    fcm_tokens += "|";
                }
            }
            msg.token = Some(fcm_tokens);
            msgs.push(msg);
        }

        Ok(msgs)
    }
}


#[cfg(feature = "wecom")]
impl<'a> TryFrom<Message<'a>> for wecom::Message<'a> {
    type Error = Error;

    fn try_from(msg: Message<'a>) -> Result<Self, Self::Error> {
        let extra = msg.wecom.as_ref();
        match msg.body {
            Body::Transparent(data) => wecom::MessageBuilder::default()
                .inner(wecom::InnerMesssage::Text { content: data })
                .build()
                .map_err(|e| InnerError::Format(e.to_string()).into()),
            Body::Notify { title, body } => {
                let mut br = wecom::MessageBuilder::default();

                if title.is_empty() {
                    br.inner(wecom::InnerMesssage::Text { content: body });
                }

                if let Some(extra) = extra {
                    match extra {
                        WecomExtra::Markdown(mark) => {
                            if *mark {
                                br.inner(wecom::InnerMesssage::Markdown { content: body });
                            }
                        }
                        WecomExtra::Text { url, btntxt } => {
                            br.inner(wecom::InnerMesssage::Textcard {
                                title,
                                description: body,
                                url,
                                btntxt: Some(btntxt),
                            });
                        }
                    }
                }
                br.build()
                    .map_err(|e| InnerError::Format(e.to_string()).into())
            }
        }
    }
}

#[cfg(feature = "wecom")]
impl<'a> FromMessage<'a> for wecom::Message<'a> {}

#[cfg(feature = "apns")]
impl<'a> FromMessage<'a> for Vec<apns::Notification<'a>> {}

#[cfg(feature = "apns")]
impl<'a> TryFrom<Message<'a>> for Vec<apns::Notification<'a>> {
    type Error = Error;

    fn try_from(msg: Message<'a>) -> Result<Self, Self::Error> {
        let extra = msg.apns.as_ref();

        let mut resps = Vec::new();

        for &token in msg.tokens {
            let mut br = apns::NotificationBuilder::new(
                extra
                    .ok_or(InnerError::Format("missing topic".to_string()))?
                    .topic,
                token,
            );

            match msg.body {
                Body::Transparent(data) => {
                    br.custom(HashMap::from_iter([("text", data)]));
                }
                Body::Notify { title, body } => {
                    br.title(title);
                    br.body(body);
                }
            }
            resps.push(br.build());
        }

        Ok(resps)
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum Visibility {
    Unspecified = 0,
    Private = 1,
    Public = 2,
    Secret = 3,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Priority {
    High,
    Normal,
}

#[derive(Debug, Clone)]
pub struct AndroidExtra<'a> {
    pub collapse_key: Option<i64>,
    pub priority: Option<Priority>,
    pub ttl: Option<i64>,
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub icon: Option<&'a str>,
    pub color: Option<&'a str>,
    pub sound: Option<&'a str>,
    pub tag: Option<&'a str>,

    // huawei required
    pub click_action: &'a str,
    pub body_loc_key: &'a str,
    pub body_loc_args: &'a [&'a str],
    pub title_loc_key: &'a str,
    pub title_loc_args: &'a [&'a str],
    pub channel_id: &'a str,
    pub image: &'a str,
    pub ticker: &'a str,
    pub visibility: Visibility,
    // xiaomi required
    pub package_name: &'a str,
    // huawei
    pub auto_clear: Option<i8>,
    // huawei and xiaomi
    pub foreground_show: Option<bool>,
    // xiaomi
    pub notify_id: Option<i32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PushType {
    Alert,
    Voip,
}

#[derive(Debug, Serialize)]
pub struct IosExtra<'a> {
    pub push_type: PushType,
    pub topic: &'a str,
}

pub enum Client<'a> {
    #[cfg(feature = "xiaomi")]
    Mi(xiaomi::Client),
    #[cfg(feature = "huawei")]
    Huawei(huawei::Client<'a>),
    #[cfg(feature = "fcm")]
    Fcm(fcm::Client),
    #[cfg(feature = "wecom")]
    Wecom(wecom::Client<'a>),
    #[cfg(feature = "apns")]
    Apns(apns::Client),
}

struct Service<'a> {
    // client_id: &'a str,
    // client_secret: &'a str,
    pushers: RwLock<HashMap<&'a str, Client<'a>>>, // ch_id <=> client
}

impl<'a> Service<'a> {
    // fn new(client_id: &'a str, client_secret: &'a str) -> Self {
    pub fn new() -> Self {
        Self {
            pushers: Default::default(),
        }
    }

    /*
        Remove client from service.
    */
    pub async fn remove_client(&self, ch_id: &'a str) {
        let mut pushers = self.pushers.write().await;
        (*pushers).remove(ch_id);
    }

    /*
        Add client to service. Repeated.
    */
    pub async fn register_client(&self, ch_id: &'a str, cli: Client<'a>) {
        let mut pushers = self.pushers.write().await;
        (*pushers).insert(ch_id, cli);
    }

    /*
        Push message to third server by client_id.
    */
    pub async fn retry_batch_push(
        &self,
        ch_id: &'a str,
        msg: Message<'_>,
    ) -> Result<PushResult, Error> {
        let pusher = self.pushers.read().await;
        let pusher = (*pusher).get(ch_id).ok_or(InnerError::NoExistClient)?;

        match pusher {
            #[cfg(feature = "xiaomi")]
            Client::Mi(client) => {
                let resp = client.push(&msg.try_into()?).await?;
                Ok(PushResult {
                    request_id: "".to_string(),
                    code: resp.code.to_string(),
                    reason: resp.description,
                    success: 0,
                    failure: 0,
                    invalid_tokens: Vec::new(),
                })
            }
            #[cfg(feature = "huawei")]
            Client::Huawei(client) => {
                let resp = client.push(&msg.try_into()?).await?;
                Ok(PushResult {
                    request_id: "".to_string(),
                    code: "".to_string(),
                    reason: "".to_string(),
                    success: 0,
                    failure: 0,
                    invalid_tokens: Vec::new(),
                })
            }
            #[cfg(feature = "fcm")]
            Client::Fcm(client) => {
                let resp = client.push(&msg.try_into()?).await?;
                Ok(PushResult {
                    request_id: "".to_string(),
                    code: "".to_string(),
                    reason: "".to_string(),
                    success: 0,
                    failure: 0,
                    invalid_tokens: Vec::new(),
                })
            }
            #[cfg(feature = "wecom")]
            Client::Wecom(client) => {
                let resp = client.push(&msg.try_into()?).await?;
                Ok(PushResult {
                    request_id: "".to_string(),
                    code: "".to_string(),
                    reason: "".to_string(),
                    success: 0,
                    failure: 0,
                    invalid_tokens: Vec::new(),
                })
            }
            #[cfg(feature = "apns")]
            Client::Apns(client) => {
                let a: Vec<apns::Notification> = msg.try_into()?;
                let resps = client.push(a.as_slice()).await?;
                let mut res = PushResult::default();
                for resp in resps {
                    if resp.status_code.is_success() {
                        res.success += 1;
                    } else {
                        res.failure += 1;
                    }
                }
                Ok(PushResult {
                    request_id: "".to_string(),
                    code: "".to_string(),
                    reason: "".to_string(),
                    success: 0,
                    failure: 0,
                    invalid_tokens: Vec::new(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use flatten_json_object::{ArrayFormatting, Flattener};

    use crate::{apns, fcm, huawei, xiaomi, Client, Service};

    #[cfg(feature = "wecom")]
    use crate::wecom;

    #[tokio::test]
    async fn test_services() {
        let client_id = std::env::var("MI_CLIENT_ID").unwrap();
        let client_secret = std::env::var("MI_CLIENT_SECRET").unwrap();
        let project_id = std::env::var("MI_PROJECT_ID").unwrap();

        let mi = xiaomi::Client::new(&xiaomi::Config {
            client_id: &client_id,
            client_secret: &client_secret,
            project_id: &project_id,
        })
            .unwrap();


        let client_id = std::env::var("WECOM_CLIENT_ID").unwrap();
        let client_secret = std::env::var("WECOM_CLIENT_SECRET").unwrap();
        let agent_id = std::env::var("WECOM_AGENT_ID").unwrap().parse::<i64>().expect("");

        #[cfg(feature = "wecom")]
            let wecom_ = wecom::Client::new(
            &client_id,
            &client_secret,
            agent_id,
        )
            .await
            .unwrap();


        let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap();
        let client_email = std::env::var("GOOGLE_CLIENT_EMAIL").unwrap();
        let private_id = std::env::var("GOOGLE_PRIVATE_ID").unwrap();
        let private_key = std::env::var("GOOGLE_PRIVATE_KEY").unwrap();
        let project_id = std::env::var("GOOGLE_PROJECT_ID").unwrap();


        let mut fcm_ = fcm::Client::new(fcm::Config {
            key_type: "service_account".to_string().into(),
            client_id: client_id.into(),
            private_key_id: private_id.into(),
            private_key,
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string().into(),
            project_id: project_id.into(),
            client_email,
            auth_provider_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".to_string()),
            client_x509_cert_url: Some("https://www.googleapis.com/robot/v1/metadata/x509/firebase-adminsdk-vle32%40avcf-7ea7e.iam.gserviceaccount.com".to_string()),
        })
            .await.unwrap();

        let apns_ = apns::Client::new(b"", "pass").unwrap();

        let hw = huawei::Client::new("", "").await.unwrap();

        let svc = Service::new();

        svc.register_client("mi", Client::Mi(mi)).await;

        #[cfg(feature = "wecom")]
        svc.register_client("wecom", Client::Wecom(wecom_)).await;

        svc.register_client("fcm", Client::Fcm(fcm_)).await;
        svc.register_client("apns", Client::Apns(apns_)).await;
        svc.register_client("hw", Client::Huawei(hw)).await;
    }

    #[tokio::test]
    async fn message_to_message() {
        // let msg = super::Message::default();
        // let xiaomi: xiaomi::Message = msg.into();
        let str = r#"
            {
                "cmd":"cmd",
                "ts":1234,
                "opt":{
                    "reqTs":123,
                    "inviter":"nekilc"
                }
            }
        "#;

        let val: serde_json::Value = serde_json::from_str(str).unwrap();

        let res = Flattener::new()
            .set_key_separator(".")
            .set_array_formatting(ArrayFormatting::Plain)
            .set_preserve_empty_arrays(false)
            .set_preserve_empty_objects(false)
            .flatten(&val)
            .unwrap();
        let res = serde_json::to_string_pretty(&res).unwrap();
        println!("{}", res);

        for group in ["1", "2", "3", "4", "5"].rsplitn(2, |e| false) {
            println!("{:?}", group);
        }
    }

    #[test]
    #[cfg(feature = "huawei")]
    fn test_huawei_msg() {
        use crate::{Message, huawei, Body};
        let mut tokens = Vec::new();
        for i in 0..1100 {
            tokens.push(i.to_string());
        }
        let mut tokens_ = Vec::new();
        for i in 0..tokens.len() {
            tokens_.push(tokens[i].as_str());
        }
        let msg = Message {
            tokens: tokens_.as_slice(),
            body: Body::Transparent(""),
            android: None,
            apns: None,
            wecom: None,
        };
        let hw_msg: huawei::Message = msg.clone().try_into().unwrap();
        assert_eq!(hw_msg.message.token.len(), 1100);

        let hw_msgs: Vec<huawei::Message> = msg.try_into().unwrap();
        assert_eq!(hw_msgs.len(), 2);
        assert_eq!(hw_msgs[0].message.token.len(), 1000);
        assert_eq!(hw_msgs[1].message.token.len(), 100);
    }

    #[test]
    #[cfg(feature = "fcm")]
    fn test_fcm_msg() {
        use crate::{Message, fcm, Body};
        let mut tokens = Vec::new();
        for i in 0..1100 {
            tokens.push(i.to_string());
        }
        let mut tokens_ = Vec::new();
        for i in 0..tokens.len() {
            tokens_.push(tokens[i].as_str());
        }
        let msg = Message {
            tokens: tokens_.as_slice(),
            body: Body::Transparent(""),
            android: None,
            apns: None,
            wecom: None,
        };
        let hw_msg: fcm::Message = msg.clone().try_into().unwrap();
        assert_eq!(hw_msg.token.unwrap().split('|').collect::<Vec<_>>().len(), 1100);

        let hw_msgs: Vec<fcm::Message> = msg.try_into().unwrap();
        assert_eq!(hw_msgs.len(), 2);
        assert_eq!(hw_msgs[0].token.as_ref().unwrap().split('|').collect::<Vec<_>>().len(), 1000);
        assert_eq!(hw_msgs[1].token.as_ref().unwrap().split('|').collect::<Vec<_>>().len(), 100);
    }
}
