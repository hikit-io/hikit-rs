#[cfg(feature = "libs")]
pub use self::lib::*;
#[cfg(feature = "libs-model")]
pub use self::model::*;

#[cfg(feature = "libs-model")]
pub mod model;

#[cfg(feature = "libs")]
mod lib {
    use std::{collections::HashMap, error::Error as StdError};

    use async_trait::async_trait;
    #[cfg(any(feature = "xiaomi", feature = "huawei", feature = "fcm"))]
    use flatten_json_object::{ArrayFormatting, Flattener};
    use http::StatusCode;
    use thiserror::Error;
    use tokio::sync::RwLock;

    #[cfg(feature = "apns")]
    use crate::apns;
    #[cfg(feature = "email")]
    use crate::email;
    #[cfg(feature = "fcm")]
    use crate::fcm;
    #[cfg(feature = "huawei")]
    use crate::huawei;
    #[cfg(feature = "wecom")]
    use crate::wecom;
    #[cfg(feature = "xiaomi")]
    use crate::xiaomi;

    use super::model::*;

    pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

    #[derive(Debug, Error)]
    #[error(transparent)]
    pub struct Error {
        inner: InnerError,
    }

    #[derive(Debug, Error)]
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
        #[error("missing required parameter: `{0}`")]
        MissingRequired(String),
        #[error("invalid parameter: `{0}`")]
        InvalidParams(String),
        #[error("no exist client")]
        NoExistClient,
        #[error("serde error: `{0}`")]
        Serde(BoxError),
        #[error("message limit")]
        MessageLimit,
        #[error("token limit")]
        TokenLimit,
        #[error("unknown error: `{0}`")]
        Unknown(String),
        #[error("auth error: `{0}`")]
        Auth(String),
        #[error("UnsupportedType error: `{0}`")]
        UnsupportedType(String),
    }

    impl From<serde_json::Error> for Error {
        fn from(e: serde_json::Error) -> Self {
            Self {
                inner: InnerError::Serde(Box::new(e)),
            }
        }
    }

    impl From<reqwest::Error> for Error {
        fn from(e: reqwest::Error) -> Self {
            let inner = if e.is_builder() {
                InnerError::Client(e.to_string())
            } else if e.is_status() {
                match e.status().unwrap() {
                    StatusCode::BAD_REQUEST => InnerError::MissingRequired(e.to_string()),
                    StatusCode::SERVICE_UNAVAILABLE => {
                        InnerError::RetryError(RetryError::Server(e.to_string()))
                    }
                    StatusCode::GATEWAY_TIMEOUT => {
                        InnerError::RetryError(RetryError::Timeout(e.to_string()))
                    }
                    StatusCode::REQUEST_TIMEOUT => {
                        InnerError::RetryError(RetryError::Timeout(e.to_string()))
                    }
                    StatusCode::UNAUTHORIZED => {
                        InnerError::RetryError(RetryError::Auth(e.to_string()))
                    }
                    _ => InnerError::Unknown(e.to_string()),
                }
            } else if e.is_timeout() {
                InnerError::RetryError(RetryError::Timeout(e.to_string()))
            } else if e.is_body() {
                InnerError::Format(e.to_string())
            } else if e.is_decode() {
                InnerError::Serde(Box::new(e))
            } else {
                InnerError::Unknown(e.to_string())
            };
            Self { inner }
        }
    }

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
        M: Sync + std::fmt::Debug,
        R: Send,
    {
        const TOKEN_LIMIT: usize = 500;

        async fn push(&self, msg: &'b M) -> Result<R, Error>;

        async fn retry_push(&self, msg: &'b M) -> Result<R, Error> {
            tracing::info!("[retry_push] message:{:?}", msg);

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

        async fn retry_batch_push(&self, msgs: &'b [M]) -> Result<Vec<R>, Error> {
            let mut resps = Vec::new();
            for msg in msgs {
                let resp = self.retry_push(msg).await?;
                resps.push(resp);
            }
            Ok(resps)
        }
    }

    pub trait FromMessage<'a>: TryFrom<Message<'a>> {}

    #[cfg(feature = "xiaomi")]
    impl<'a> TryFrom<Message<'a>> for xiaomi::Message<'a> {
        type Error = Error;

        fn try_from(msg: Message<'a>) -> Result<Self, Self::Error> {
            let extra = msg.android.as_ref();
            Ok(match msg.body {
                Body::Transparent(data) => xiaomi::Message {
                    payload: Some(data),
                    pass_through: xiaomi::Passtrough::Transparent,
                    registration_id: Some(msg.tokens.join(",")),
                    ..Default::default()
                },
                Body::Notify { title, body, .. } => xiaomi::Message {
                    title,
                    description: body,
                    registration_id: Some(msg.tokens.join(",")),
                    pass_through: xiaomi::Passtrough::Notice,
                    restricted_package_name: extra.map_or("", |v| v.package_name.unwrap_or("")),
                    extra: xiaomi::Extra {
                        notify_foreground: extra
                            .map_or(None, |v| v.foreground_show.map(|v| v.into())),
                        notify_effect: extra.map_or(None, |v| match v.click_action {
                            None => None,
                            Some(click_action) => {
                                if click_action.is_empty() {
                                    None
                                } else {
                                    Some(xiaomi::NotifyEffect::Intent)
                                }
                            }
                        }),
                        intent_uri: extra.map_or(None, |v| match v.click_action {
                            None => None,
                            Some(click_action) => {
                                if click_action.is_empty() {
                                    None
                                } else {
                                    Some(click_action)
                                }
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

    #[cfg(feature = "xiaomi")]
    impl<'a> FromMessage<'a> for xiaomi::Message<'a> {}

    #[cfg(feature = "xiaomi")]
    impl<'a> FromMessage<'a> for Vec<xiaomi::Message<'a>> {}

    #[cfg(feature = "xiaomi")]
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
                msg.registration_id = Some(mi_tokens.join(","));
                msgs.push(msg);
            }

            Ok(msgs)
        }
    }

    #[cfg(feature = "huawei")]
    impl<'a> FromMessage<'a> for huawei::Message<'a> {}

    #[cfg(feature = "huawei")]
    impl<'a> TryFrom<Message<'a>> for huawei::Message<'a> {
        type Error = Error;

        fn try_from(msg: Message<'a>) -> Result<Self, Self::Error> {
            let extra = msg.android.as_ref();

            match msg.body {
                Body::Transparent(data) => Ok(huawei::Message {
                    validate_only: false,
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
                    validate_only: false,
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
                                image: extra.map_or(None, |e| e.image),
                                tag: extra.map_or(None, |e| e.tag),
                                body_loc_key: extra.map_or(None, |e| e.body_loc_key),
                                body_loc_args: extra
                                    .map_or(Default::default(), |e| e.body_loc_args),
                                title_loc_key: extra
                                    .map_or(Default::default(), |e| e.title_loc_key),
                                title_loc_args: extra
                                    .map_or(Default::default(), |e| e.title_loc_args),
                                channel_id: extra.map_or(None, |e| e.channel_id),
                                ticker: extra.map_or(None, |e| e.ticker),
                                click_action: extra
                                    .map_or(None, |e| {
                                        e.click_action.map(|e| huawei::ClickAction::new_intent(e))
                                    })
                                    .ok_or(InnerError::MissingRequired(
                                        "missing click action".to_string(),
                                    ))?,
                                visibility: extra.map_or(None, |e| {
                                    match e.visibility.as_ref().unwrap_or(&Visibility::Unspecified)
                                    {
                                        &Visibility::Unspecified => Some(""),
                                        &Visibility::Private => Some("PRIVATE"),
                                        &Visibility::Public => Some("PUBLIC"),
                                        &Visibility::Secret => Some("SECRET"),
                                    }
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

    #[cfg(feature = "huawei")]
    impl<'a> FromMessage<'a> for Vec<huawei::Message<'a>> {}

    #[cfg(feature = "huawei")]
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

    #[cfg(feature = "fcm")]
    impl<'a> FromMessage<'a> for fcm::MulticastMessage<'a> {}

    #[cfg(feature = "fcm")]
    impl<'a> TryFrom<Message<'a>> for fcm::MulticastMessage<'a> {
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
                        serde_json::from_value::<HashMap<String, String>>(val)?
                    } else {
                        HashMap::from_iter([("text".to_string(), data.to_string())])
                    };
                    Ok(fcm::MulticastMessage {
                        data: Some(data),
                        tokens: msg.tokens.to_vec(),
                        ..Default::default()
                    })
                }
                Body::Notify { title, body } => Ok(fcm::MulticastMessage {
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
                        ttl: extra
                            .map_or(None, |e| e.ttl.map_or(None, |e| Some(format!("{}s", e)))),
                        notification: Some(fcm::AndroidNotification {
                            sound: extra
                                .map_or(None, |e| e.sound.map_or(None, |e| Some(e.to_string()))),
                            icon: extra
                                .map_or(None, |e| e.icon.map_or(None, |e| Some(e.to_string()))),
                            image: extra.map_or(None, |e| match e.image {
                                None => None,
                                Some(e) => Some(e.to_string()),
                            }),
                            tag: extra
                                .map_or(None, |e| e.tag.map_or(None, |e| Some(e.to_string()))),
                            body_loc_key: extra.map_or(None, |e| match e.body_loc_key {
                                None => None,
                                Some(e) => Some(e.to_string()),
                            }),
                            body_loc_args: extra.map_or(None, |e| match e.body_loc_args {
                                None => None,
                                Some(e) => Some(e.to_vec()),
                            }),
                            title_loc_key: extra.map_or(None, |e| match e.title_loc_key {
                                None => None,
                                Some(e) => Some(e.to_string()),
                            }),
                            title_loc_args: extra.map_or(None, |e| match e.title_loc_args {
                                None => None,
                                Some(e) => Some(e.to_vec()),
                            }),
                            channel_id: extra.map_or(None, |e| match e.channel_id {
                                None => None,
                                Some(e) => Some(e.to_string()),
                            }),
                            ticker: extra.map_or(None, |e| match e.ticker {
                                None => None,
                                Some(e) => Some(e.to_string()),
                            }),
                            click_action: extra.map_or(None, |e| match e.click_action {
                                None => None,
                                Some(e) => Some(e.to_string()),
                            }),
                            visibility: extra.map_or(None, |e| match &e.visibility {
                                None => Some("".to_string()),
                                Some(e) => match e {
                                    &Visibility::Unspecified => Some("".to_string()),
                                    &Visibility::Private => Some("PRIVATE".to_string()),
                                    &Visibility::Public => Some("PUBLIC".to_string()),
                                    &Visibility::Secret => Some("SECRET".to_string()),
                                },
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

    #[cfg(feature = "fcm")]
    impl<'a> FromMessage<'a> for Vec<fcm::MulticastMessage<'a>> {}

    #[cfg(feature = "fcm")]
    impl<'a> TryFrom<Message<'a>> for Vec<fcm::MulticastMessage<'a>> {
        type Error = Error;

        fn try_from(msg: Message<'a>) -> Result<Self, Self::Error> {
            let tokens = msg.tokens;

            let hw_msg: fcm::MulticastMessage = msg.try_into()?;

            let mut msgs = Vec::new();

            for i in 0..(tokens.len() / 1000) + 1 {
                let mut fcm_tokens = Vec::new();

                let mut msg = hw_msg.clone();

                let start_index = i * 1000;
                let end_index = if tokens.len() - start_index >= 1000 {
                    1000
                } else {
                    tokens.len()
                };
                for j in start_index..end_index {
                    fcm_tokens.push(tokens[j]);
                }
                msg.tokens = fcm_tokens;
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

                    br.to(wecom::To::Touser(msg.tokens.join("|")));

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

    #[cfg(feature = "email")]
    impl<'a> FromMessage<'a> for email::Message<'a> {}

    #[cfg(feature = "email")]
    impl<'a> TryFrom<Message<'a>> for email::Message<'a> {
        type Error = Error;

        fn try_from(msg: Message<'a>) -> Result<Self, Self::Error> {
            match msg.body {
                Body::Transparent(_) => {
                    Err(InnerError::UnsupportedType("Transparent".to_string()).into())
                }
                Body::Notify { title, body } => Ok(Self {
                    title: title,
                    body: body,
                    to: msg.tokens,
                }),
            }
        }
    }

    pub enum Client {
        #[cfg(feature = "xiaomi")]
        Mi(xiaomi::Client),
        #[cfg(feature = "huawei")]
        Huawei(huawei::Client),
        #[cfg(feature = "fcm")]
        Fcm(fcm::Client),
        #[cfg(feature = "wecom")]
        Wecom(wecom::Client),
        #[cfg(feature = "apns")]
        Apns(apns::Client),
        #[cfg(feature = "email")]
        Email(email::Client),
    }

    pub struct Service {
        pub(crate) pushers: RwLock<HashMap<String, Client>>, // ch_id <=> client
    }

    impl Service {
        // fn new(client_id: &'a str, client_secret: &'a str) -> Self {
        pub fn new() -> Self {
            Self {
                pushers: Default::default(),
            }
        }

        /*
            Remove client from service.
        */
        pub async fn remove_client(&self, ch_id: &str) {
            let mut pushers = self.pushers.write().await;
            (*pushers).remove(ch_id);
        }

        /*
            Add client to service. Repeated.
        */
        pub async fn register_client(&self, ch_id: &str, cli: Client) {
            let mut pushers = self.pushers.write().await;
            (*pushers).insert(ch_id.to_string(), cli);
        }

        /*
            Push message to third server by client_id.
        */
        pub async fn retry_batch_push(
            &self,
            ch_id: &str,
            msg: Message<'_>,
        ) -> Result<PushResults, Error> {
            let pusher = self.pushers.read().await;
            let pusher = (*pusher).get(ch_id).ok_or(InnerError::NoExistClient)?;

            match pusher {
                #[cfg(feature = "xiaomi")]
                Client::Mi(client) => {
                    let token_len = msg.tokens.len();
                    let msg: Vec<xiaomi::Message> = msg.try_into()?;
                    let resp = client.retry_batch_push(msg.as_slice()).await?;

                    let mut res = PushResults::default();
                    for resp in resp.into_iter() {
                        res.results.push(PushResult {
                            request_id: resp.data.id,
                            code: resp.code.to_string(),
                            reason: resp.reason,
                            invalid_tokens: Vec::new(),
                            ..Default::default()
                        });
                    }
                    res.success = token_len as i64;
                    Ok(res)
                }
                #[cfg(feature = "huawei")]
                Client::Huawei(client) => {
                    let msg: Vec<huawei::Message> = msg.try_into()?;
                    let resp = client.retry_batch_push(msg.as_slice()).await?;
                    let mut res = PushResults::default();
                    for resp in resp.into_iter() {
                        res.results.push(PushResult {
                            request_id: resp.request_id,
                            code: serde_json::to_string(&resp.code).unwrap(),
                            reason: resp.msg,
                            success: resp.success,
                            failure: resp.failure,
                            invalid_tokens: Vec::new(),
                        });
                        res.success += resp.success;
                        res.failure += resp.failure;
                    }
                    Ok(res)
                }
                #[cfg(feature = "fcm")]
                Client::Fcm(client) => {
                    let msg: Vec<fcm::MulticastMessage> = msg.try_into()?;
                    let resp = client.retry_batch_push(msg.as_slice()).await?;

                    let mut res = PushResults::default();
                    for resp in resp.into_iter() {
                        for respons in resp.responses {
                            match respons {
                                fcm::SendResponse::Ok { message_id } => {
                                    res.results.push(PushResult {
                                        request_id: message_id,
                                        ..Default::default()
                                    });
                                }
                                fcm::SendResponse::Error { error, token } => {
                                    res.results.push(PushResult {
                                        invalid_tokens: vec![token],
                                        reason: error,
                                        ..Default::default()
                                    });
                                }
                            }
                        }
                        res.success += resp.success_count;
                        res.failure += resp.failure_count;
                    }
                    Ok(res)
                }
                #[cfg(feature = "wecom")]
                Client::Wecom(client) => {
                    let token_len = msg.tokens.len();
                    let msg: wecom::Message = msg.try_into()?;

                    let resp = client.retry_push(&msg).await?;

                    let invalid_tokens = resp.invaliduser.unwrap_or_default();
                    let invalid_tokens = if !invalid_tokens.is_empty() {
                        invalid_tokens
                            .split("|")
                            .map(|e| e.to_string())
                            .collect::<Vec<String>>()
                    } else {
                        vec![]
                    };

                    let res = PushResult {
                        request_id: resp.msgid,
                        code: resp.errcode.to_string(),
                        reason: resp.errmsg,
                        success: (token_len - invalid_tokens.len()) as i64,
                        failure: invalid_tokens.len() as i64,
                        invalid_tokens,
                    };

                    Ok(PushResults {
                        success: res.success,
                        failure: res.failure,
                        results: vec![res],
                    })
                }
                #[cfg(feature = "apns")]
                Client::Apns(client) => {
                    let a: Vec<apns::Notification> = msg.try_into()?;
                    let resps = client.retry_batch_push(a.as_slice()).await?;
                    let mut res = PushResults::default();
                    for resp in resps {
                        if resp.status_code.is_success() {
                            res.results.push(PushResult {
                                request_id: resp.apns_id,
                                code: resp.status_code.to_string(),
                                reason: resp.reason.to_str().to_string(),
                                success: 1,
                                failure: 0,
                                invalid_tokens: vec![],
                            });
                            res.success += 1;
                        } else {
                            res.results.push(PushResult {
                                request_id: resp.apns_id,
                                code: resp.status_code.to_string(),
                                reason: resp.reason.to_str().to_string(),
                                success: 0,
                                failure: 1,
                                invalid_tokens: vec![resp.token],
                            });
                            res.failure += 1;
                        }
                    }
                    Ok(res)
                }
                #[cfg(feature = "email")]
                Client::Email(client) => {
                    let mut res = PushResults::default();
                    let msg = msg.try_into()?;
                    let resp = client.retry_push(&msg).await?;
                    for resp in resp.results {
                        if resp.success {
                            res.success += 1;
                            res.results.push(PushResult {
                                success: 1,
                                ..Default::default()
                            });
                        } else {
                            res.failure += 1;
                            res.results.push(PushResult {
                                failure: 1,
                                invalid_tokens: vec![resp.email.to_string()],
                                reason: resp.reason.unwrap_or_default(),
                                ..Default::default()
                            });
                        }
                    }
                    Ok(res)
                }
            }
        }
    }
}

//
// #[cfg(test)]
// mod tests {
//     use flatten_json_object::{ArrayFormatting, Flattener};
//
//     use crate::{apns, fcm, huawei, xiaomi, Client, Service};
//
//     #[cfg(feature = "wecom")]
//     use crate::wecom;
//
//     #[tokio::test]
//     async fn test_services() {
//         let client_id = std::env::var("MI_CLIENT_ID").unwrap();
//         let client_secret = std::env::var("MI_CLIENT_SECRET").unwrap();
//         let project_id = std::env::var("MI_PROJECT_ID").unwrap();
//
//         let mi = xiaomi::Client::new(&xiaomi::Config {
//             client_id: &client_id,
//             client_secret: &client_secret,
//             project_id: &project_id,
//         })
//             .unwrap();
//
//         let client_id = std::env::var("WECOM_CLIENT_ID").unwrap();
//         let client_secret = std::env::var("WECOM_CLIENT_SECRET").unwrap();
//         let agent_id = std::env::var("WECOM_AGENT_ID")
//             .unwrap()
//             .parse::<i64>()
//             .expect("");
//
//         #[cfg(feature = "wecom")]
//             let wecom_ = wecom::Client::new(&client_id, &client_secret, agent_id)
//             .await
//             .unwrap();
//
//         let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap();
//         let client_email = std::env::var("GOOGLE_CLIENT_EMAIL").unwrap();
//         let private_id = std::env::var("GOOGLE_PRIVATE_ID").unwrap();
//         let private_key = std::env::var("GOOGLE_PRIVATE_KEY").unwrap();
//         let project_id = std::env::var("GOOGLE_PROJECT_ID").unwrap();
//
//         let mut fcm_ = fcm::Client::new(fcm::Config {
//             key_type: "service_account".to_string().into(),
//             client_id: client_id.into(),
//             private_key_id: private_id.into(),
//             private_key,
//             token_uri: "https://oauth2.googleapis.com/token".to_string(),
//             auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string().into(),
//             project_id: project_id.into(),
//             client_email,
//             auth_provider_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".to_string()),
//             client_x509_cert_url: Some("https://www.googleapis.com/robot/v1/metadata/x509/firebase-adminsdk-vle32%40avcf-7ea7e.iam.gserviceaccount.com".to_string()),
//         })
//             .await.unwrap();
//
//         fcm_.with_proxy(fcm::ProxyConfig {
//             addr: "socks5://127.0.0.1:7890",
//             user: None,
//             pass: None,
//         })
//             .await;
//
//         let apns_ = apns::Client::new(b"", "pass").unwrap();
//
//         let hw = huawei::Client::new("", "").await.unwrap();
//
//         let svc = Service::new();
//
//         svc.register_client("mi", Client::Mi(mi)).await;
//
//         #[cfg(feature = "wecom")]
//         svc.register_client("wecom", Client::Wecom(wecom_)).await;
//
//         svc.register_client("fcm", Client::Fcm(fcm_)).await;
//         svc.register_client("apns", Client::Apns(apns_)).await;
//         svc.register_client("hw", Client::Huawei(hw)).await;
//     }
//
//     #[tokio::test]
//     async fn message_to_message() {
//         // let msg = super::Message::default();
//         // let xiaomi: xiaomi::Message = msg.into();
//         let str = r#"
//             {
//                 "cmd":"cmd",
//                 "ts":1234,
//                 "opt":{
//                     "reqTs":123,
//                     "inviter":"nekilc"
//                 }
//             }
//         "#;
//
//         let val: serde_json::Value = serde_json::from_str(str).unwrap();
//
//         let res = Flattener::new()
//             .set_key_separator(".")
//             .set_array_formatting(ArrayFormatting::Plain)
//             .set_preserve_empty_arrays(false)
//             .set_preserve_empty_objects(false)
//             .flatten(&val)
//             .unwrap();
//         let res = serde_json::to_string_pretty(&res).unwrap();
//         println!("{}", res);
//
//         for group in ["1", "2", "3", "4", "5"].rsplitn(2, |e| false) {
//             println!("{:?}", group);
//         }
//     }
//
//     #[test]
//     #[cfg(feature = "huawei")]
//     fn test_huawei_msg() {
//         use crate::{huawei, Body, Message};
//         let mut tokens = Vec::new();
//         for i in 0..1100 {
//             tokens.push(i.to_string());
//         }
//         let mut tokens_ = Vec::new();
//         for i in 0..tokens.len() {
//             tokens_.push(tokens[i].as_str());
//         }
//         let msg = Message {
//             tokens: tokens_.as_slice(),
//             body: Body::Transparent(""),
//             #[cfg(feature = "wecom")]
//             android: None,
//             apns: None,
//             wecom: None,
//         };
//         let hw_msg: huawei::Message = msg.clone().try_into().unwrap();
//         assert_eq!(hw_msg.message.token.len(), 1100);
//
//         let hw_msgs: Vec<huawei::Message> = msg.try_into().unwrap();
//         assert_eq!(hw_msgs.len(), 2);
//         assert_eq!(hw_msgs[0].message.token.len(), 1000);
//         assert_eq!(hw_msgs[1].message.token.len(), 100);
//     }
//
//     #[test]
//     #[cfg(feature = "fcm")]
//     fn test_fcm_msg() {
//         use crate::{fcm, Body, Message};
//         let mut tokens = Vec::new();
//         for i in 0..1100 {
//             tokens.push(i.to_string());
//         }
//         let mut tokens_ = Vec::new();
//         for i in 0..tokens.len() {
//             tokens_.push(tokens[i].as_str());
//         }
//         let msg = Message {
//             tokens: tokens_.as_slice(),
//             body: Body::Transparent(""),
//             android: None,
//             apns: None,
//             wecom: None,
//         };
//         let hw_msg: fcm::MulticastMessage = msg.clone().try_into().unwrap();
//         assert_eq!(hw_msg.tokens.len(), 1100);
//
//         let hw_msgs: Vec<fcm::MulticastMessage> = msg.try_into().unwrap();
//         assert_eq!(hw_msgs.len(), 2);
//         assert_eq!(hw_msgs[0].tokens.len(), 1000);
//         assert_eq!(hw_msgs[1].tokens.len(), 100);
//     }
// }
