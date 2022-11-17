use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[cfg(feature = "mongo")]
use mongodb::bson::oid::ObjectId;

#[cfg(any(feature = "apns-model", feature = "client"))]
use crate::apns;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Admin {
    #[cfg(feature = "mongo")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub password: String,
    pub update_ts: i64,
    pub create_ts: i64,
}

#[cfg(not(feature = "client"))]
#[derive(Debug, Serialize_repr, Deserialize_repr, Default, Clone)]
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
}

/*
    App
*/
#[cfg(not(feature = "client"))]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct App {
    #[cfg(feature = "mongo")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
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
#[cfg(not(feature = "client"))]
#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[cfg(not(feature = "client"))]
impl Default for Channel {
    fn default() -> Self {
        Self {
            #[cfg(feature = "mongo")]
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
#[cfg(not(feature = "client"))]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[cfg(feature = "mongo")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub app_id: String,
    pub ch_id: String,
    pub group: String,
    pub token: String,
    pub valid: bool,
    pub create_ts: i64,
    pub update_ts: i64,
}

#[cfg(not(feature = "client"))]
impl Default for Token {
    fn default() -> Self {
        Self {
            #[cfg(feature = "mongo")]
            id: Default::default(),
            ch_id: Default::default(),
            token: Default::default(),
            valid: true,
            create_ts: chrono::Utc::now().timestamp(),
            update_ts: chrono::Utc::now().timestamp(),
            group: Default::default(),
            app_id: Default::default(),
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

#[cfg(not(feature = "client"))]
#[derive(Debug, Serialize, Builder, Clone, Default)]
pub struct Response<T = String> {
    #[builder(default)]
    pub code: Code,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[builder(default)]
    pub msg: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

#[cfg(not(feature = "client"))]
impl<T> From<anyhow::Error> for Response<T> {
    fn from(e: anyhow::Error) -> Self {
        Self {
            code: Code::Err,
            data: Option::<T>::None,
            msg: e.to_string(),
            errors: None,
        }
    }
}

#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(not(feature = "client"), derive(Serialize, Clone))]
#[serde(rename_all = "camelCase")]
pub struct RegisterTokenResp {
    pub success: u64,
    pub failure: u64,
    pub failure_tokens: Vec<String>,
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct RegisterTokenParams {
    pub group: String,
    pub token: String,
    pub ch_id: String,
    #[serde(rename = "override")]
    pub _override: Option<bool>,
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct RevokeTokenParams {
    pub group: String,
    pub token: String,
    pub ch_id: String,
}

#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(not(feature = "client"), derive(Serialize, Clone))]
#[serde(rename_all = "camelCase")]
pub struct RevokeTokenResp {
    pub success: u64,
    pub failure: u64,
    pub failure_tokens: Vec<String>,
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub channels: Vec<String>,
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub condition: Condition,
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub enum Body {
    Json(serde_json::Map<String, serde_json::Value>),
    Text(String),
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct PushTransparentParams {
    pub groups: Vec<String>,
    pub channels: Vec<String>,
    #[cfg_attr(not(feature = "client"), serde(flatten))]
    pub body: Body,
    #[cfg_attr(not(feature = "client"), serde(flatten))]
    pub platform_extra: PlatformParams,
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct PushNotificationParams {
    pub groups: Vec<String>,
    pub channels: Vec<String>,
    pub title: String,
    pub body: String,
    #[cfg_attr(not(feature = "client"), serde(flatten))]
    pub platform_extra: PlatformParams,
}

#[cfg(any(feature = "wecom", feature = "client"))]
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub enum WecomExtra {
    Markdown(bool),
    Text { url: String, btntxt: String },
}

#[cfg(any(feature = "apns", feature = "client"))]
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct ApnsExtra {
    pub topic: String,
    pub push_type: apns::ApnsPushType,
}

#[cfg(all(
any(
feature = "fcm",
feature = "xiaomi",
feature = "huawei",
feature = "client",
),
not(all(feature = "client", target_arch = "wasm32"))
))]
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct AndroidExtra {
    pub collapse_key: Option<i64>,
    pub priority: Option<crate::Priority>,
    pub ttl: Option<i64>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub sound: Option<String>,
    pub tag: Option<String>,

    // huawei required
    pub click_action: Option<String>,
    pub body_loc_key: Option<String>,
    pub body_loc_args: Option<Vec<String>>,
    pub title_loc_key: Option<String>,
    pub title_loc_args: Option<Vec<String>>,
    pub channel_id: Option<String>,
    pub image: Option<String>,
    pub ticker: Option<String>,
    pub visibility: Option<crate::Visibility>,
    // xiaomi required
    pub package_name: Option<String>,
    // huawei
    pub auto_clear: Option<i8>,
    // huawei and xiaomi
    pub foreground_show: Option<bool>,
    // xiaomi
    pub notify_id: Option<i32>,
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct PlatformParams {
    #[cfg(any(
    feature = "fcm",
    feature = "xiaomi",
    feature = "huawei",
    feature = "client"
    ))]
    pub android: Option<AndroidExtra>,
    #[cfg(any(feature = "apns", feature = "client"))]
    pub apns: Option<ApnsExtra>,
    #[cfg(any(feature = "wecom", feature = "client"))]
    pub wecom: Option<WecomExtra>,
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub enum PublicChannel {
    #[cfg(any(feature = "wecom", feature = "client"))]
    #[serde(rename_all = "camelCase")]
    Wecom {
        client_id: String,
        client_secret: String,
        agentid: i64,
    },
    #[cfg(any(feature = "fcm", feature = "client"))]
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
    #[cfg(any(feature = "email", feature = "client"))]
    #[serde(rename_all = "camelCase")]
    Email {
        client_id: String,
        client_secret: String,
        addr: String,
    },
    #[cfg(any(feature = "xiaomi", feature = "client"))]
    #[serde(rename_all = "camelCase")]
    Xiaomi {
        client_id: String,
        client_secret: String,
    },
    #[cfg(any(feature = "apns", feature = "client"))]
    #[serde(rename_all = "camelCase")]
    Apns {
        client_id: String,
        client_secret: String,
    },
    #[cfg(any(feature = "huawei", feature = "client"))]
    #[serde(rename_all = "camelCase")]
    Huawei {
        client_id: String,
        client_secret: String,
    },
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Running {
    pub ch_ids: Vec<String>,
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct CreateAppParams {
    pub name: String,
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct DeleteChannelParams {
    pub ch_id: String,
}

#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(not(feature = "client"), derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub struct PushResp {
    pub success: i64,
    pub failure: i64,
    pub results: HashMap<String, crate::PushResults>,
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(not(feature = "client"), derive(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct DeleteApplicationParams {
    pub client_id: String,
}
