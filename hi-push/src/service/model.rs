use derive_builder::Builder;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr, Default)]
#[repr(i32)]
pub enum ChannelType {
    #[default]
    Unknown = 0,
    #[cfg(feature = "xiaomi")]
    Mi = 1,
    #[cfg(feature = "huawei")]
    Huawei = 2,
    #[cfg(feature = "fcm")]
    Fcm = 3,
    #[cfg(feature = "wecom")]
    Wecom = 4,
    #[cfg(feature = "apns")]
    Apns = 5,
    #[cfg(feature = "email")]
    Email = 6,
    #[cfg(feature = "rtm")]
    Rtm = 7,
}

/*
    App
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
    #[cfg(feature = "mongo")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub client_id: String,
    pub client_secret: String,

    pub create_ts: i64,
    pub update_ts: i64,
}

/*
    Channel
    key_type: todo!(),
    project_id: Some(conf.project_id.to_string()),
    private_key_id: todo!(),
    private_key: todo!(),
    client_email: todo!(),
    client_id: todo!(),
    auth_uri: todo!(),
    token_uri: todo!(),
    auth_provider_x509_cert_url: todo!(),
    client_x509_cert_url: todo!(),
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    #[cfg(feature = "mongo")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub ch_id: String,
    pub app_id: String,
    #[serde(rename = "type")]
    pub _type: ChannelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    //
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    // apple
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certs: Option<Vec<u8>>,
    // wecom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agentid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_provider_x509_cert_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_x509_cert_url: Option<String>,
    // email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,

    pub create_ts: i64,
    pub update_ts: i64,
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            id: Default::default(),
            ch_id: uuid::Uuid::new_v4().to_string(),
            app_id: "".to_string(),
            _type: Default::default(),
            client_id: None,
            client_secret: None,
            project_id: None,
            certs: None,
            agentid: None,
            key_type: None,
            private_key_id: None,
            private_key: None,
            client_email: None,
            auth_uri: None,
            token_uri: None,
            auth_provider_x509_cert_url: None,
            client_x509_cert_url: None,
            addr: None,
            create_ts: chrono::Utc::now().timestamp(),
            update_ts: chrono::Utc::now().timestamp(),
        }
    }
}

/*
    Token
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[cfg(feature = "mongo")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub ch_id: String,
    pub group: String,
    pub token: String,
    pub valid: bool,
    pub create_ts: i64,
    pub update_ts: i64,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            id: Default::default(),
            ch_id: Default::default(),
            token: Default::default(),
            valid: true,
            create_ts: chrono::Utc::now().timestamp(),
            update_ts: chrono::Utc::now().timestamp(),
            group: Default::default(),
        }
    }
}

/**
 *
 * Data transfer object
 */
#[derive(Debug, Serialize_repr, Clone, Default)]
#[repr(u8)]
pub enum Code {
    #[default]
    Ok,
    Err,
}

#[derive(Debug, Serialize, Builder, Clone, Default)]
pub struct Response<T = String> {
    #[builder(default)]
    pub code: Code,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[builder(default)]
    pub msg: String,
}

impl<T> From<anyhow::Error> for Response<T> {
    fn from(e: anyhow::Error) -> Self {
        Self {
            code: Code::Err,
            data: None,
            msg: e.to_string(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct RegisterTokenResp {
    pub(crate) success: u64,
    pub(crate) failure: u64,
    pub failure_tokens: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterTokenParams {
    pub group: String,
    pub token: String,
    pub ch_id: String,
    #[serde(rename = "override")]
    pub _override: Option<bool>,
}

#[derive(Deserialize)]
pub struct RevokeTokenParams {
    pub group: String,
    pub token: String,
    pub chan: String,
}

#[derive(Serialize, Clone)]
pub struct RevokeTokenResp {
    pub(crate) success: u64,
    pub(crate) failure: u64,
    pub failure_tokens: Vec<String>,
}

#[derive(Deserialize)]
pub struct Condition {
    pub channels: Vec<String>,
}

#[derive(Deserialize)]
pub struct Options {
    pub condition: Condition,
}

#[derive(Deserialize)]
pub enum Body {
    Json(serde_json::Map<String, serde_json::Value>),
    Text(String),
}

#[derive(Deserialize)]
pub struct PushTransparentParams {
    pub groups: Vec<String>,
    pub channels: Vec<String>,
    #[serde(flatten)]
    pub body: Body,
    #[serde(flatten)]
    pub platform_extra: PlatformParams,
}

#[derive(Deserialize)]
pub struct PushNotificationParams {
    pub groups: Vec<String>,
    pub channels: Vec<String>,
    pub title: String,
    pub body: String,
    #[serde(flatten)]
    pub platform_extra: Option<PlatformParams>,
}

#[derive(Deserialize)]
pub struct PlatformParams {
    pub android: String,
    pub apns: String,
    pub wecom: String,
    pub rtm: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PublicChannel {
    #[serde(rename_all = "camelCase")]
    Wecom {
        client_id: String,
        client_secret: String,
        agentid: i64,
    },
    #[serde(rename_all = "camelCase")]
    Fcm {
        key_type: String,
        private_key_id: String,
        private_key: String,
        client_email: String,
        auth_uri: String,
        token_uri: String,
        auth_provider_x509_cert_url: String,
        client_x509_cert_url: String,
    },
    #[serde(rename_all = "camelCase")]
    Email {
        client_id: String,
        client_secret: String,
        addr: String,
    },
    #[serde(rename_all = "camelCase")]
    Xiaomi {
        client_id: String,
        client_secret: String,
    },
    #[serde(rename_all = "camelCase")]
    Apns {
        client_id: String,
        client_secret: String,
    },
    #[serde(rename_all = "camelCase")]
    Huawei {
        client_id: String,
        client_secret: String,
    },
}

#[derive(Debug, Serialize, Clone)]
pub struct Running {
    pub(crate) ch_ids: Vec<String>,
}
