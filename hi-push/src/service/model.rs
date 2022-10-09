use derive_builder::Builder;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ChannleType {
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
}

/*
    App
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    #[cfg(feature = "mongo")]
    pub id: ObjectId,
    pub cliend_id: String,
    pub client_secret: String,
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
pub struct Channel {
    #[cfg(feature = "mongo")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub ch_id: String,
    pub app_id: String,
    //
    pub _type: ChannleType,

    pub client_id: String,
    pub cliend_secret: String,
    //
    pub project_id: String,
    // apple
    pub certs: Vec<u8>,
    // wecom
    pub agentid: i64,

    pub key_type: String,
    pub priviate_key_id: String,
    pub priviate_key: String,
    pub client_email: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_x509_cert_url: String,
}

/*
    Token
*/
#[derive(Debug, Serialize, Deserialize)]
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
 * Data tranfer object
 */
#[derive(Serialize, Clone, Default)]
#[repr(u8)]
pub enum Code {
    #[default]
    Ok,
}

#[derive(Serialize, Builder, Clone)]
pub struct Response<'a, T = &'a str> {
    code: Code,
    data: T,
    msg: &'a str,
}

#[derive(Serialize, Clone)]
pub struct RegisterTokenResp {}

#[derive(Deserialize)]
pub struct RegisterTokenParams {
    pub group: String,
    pub token: String,
    pub chan: String,
    pub overside: Option<bool>,
}

#[derive(Deserialize)]
pub struct Conditions {
    pub chans: Vec<String>,
}

#[derive(Deserialize)]
pub struct Options {
    pub conds: Conditions,
}

#[derive(Deserialize)]
pub enum Body {
    Json(serde_json::Map<String, serde_json::Value>),
    Text(String),
}

#[derive(Deserialize)]
pub struct PushTransparentParams {
    pub groups: Vec<String>,
    pub chans: Vec<String>,
    #[serde(flatten)]
    pub body: Body,
    #[serde(flatten)]
    pub platform_extra: PlatformParams,
}

#[derive(Deserialize)]
pub struct PushNotificationParams {
    pub groups: Vec<String>,
    pub chans: Vec<String>,
    pub title: String,
    pub body: String,
    #[serde(flatten)]
    pub platform_extra: PlatformParams,
}

#[derive(Deserialize)]
pub struct PlatformParams {
    pub andorid: String,
    pub apns: String,
    pub wecom: String,
    pub rtm: String,
}

pub enum PublicChannel {
    Wecom {},
    Fcm {},
    Email {},
    Xiaomi {},
    Apns {},
    Huawei {},
}
