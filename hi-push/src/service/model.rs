use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[cfg(any(feature = "apns-model", feature = "client"))]
use crate::apns;

pub use super::common_model::*;
#[cfg(all(feature = "client", target_arch = "wasm32"))]
pub use super::model_wasm::*;

#[derive(Debug, Clone, Default)]
#[cfg_attr(all(not(feature = "client")), derive(Serialize, Builder))]
#[cfg_attr(all(feature = "client"), derive(Deserialize))]
pub struct Response<T = String> {
    #[cfg_attr(all(not(feature = "client")), builder(default))]
    pub code: Code,
    #[cfg_attr(all(not(feature = "client")), builder(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[cfg_attr(all(not(feature = "client")), builder(default))]
    pub msg: String,
    #[cfg_attr(all(not(feature = "client")), builder(default))]
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

#[derive(Debug, Default)]
pub struct ResponseError {
    pub code: Code,
    pub msg: String,
    pub errors: Vec<String>,
}

impl ResponseError {
    pub fn unknown(msg: &str) -> Self {
        Self {
            code: Code::Unknown,
            msg: msg.to_string(),
            errors: vec![],
        }
    }
}

impl From<reqwest::Error> for ResponseError {
    fn from(e: reqwest::Error) -> Self {
        Self {
            code: Code::System,
            msg: e.to_string(),
            errors: vec![],
        }
    }
}

impl<T> From<Response<T>> for ResponseError {
    fn from(resp: Response<T>) -> Self {
        Self {
            code: resp.code,
            msg: resp.msg,
            errors: resp.errors.unwrap(),
        }
    }
}

#[derive(Debug)]
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

#[cfg(all(any(
    feature = "fcm",
    feature = "xiaomi",
    feature = "huawei",
    feature = "client",
)))]
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
pub struct CreateApplicationParams {
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
