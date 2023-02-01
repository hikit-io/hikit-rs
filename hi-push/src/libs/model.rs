use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
#[cfg(all(feature = "client", target_arch = "wasm32"))]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(any(feature = "apns", feature = "client"))]
use crate::apns;

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
#[cfg(any(
    feature = "xiaomi",
    feature = "huawei",
    feature = "fcm",
    feature = "client"
))]
#[cfg_attr(all(feature = "client", target_arch = "wasm32"), wasm_bindgen)]
pub enum Visibility {
    Unspecified = 0,
    Private = 1,
    Public = 2,
    Secret = 3,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
#[cfg(any(
    feature = "xiaomi",
    feature = "huawei",
    feature = "fcm",
    feature = "client"
))]
#[cfg_attr(all(feature = "client", target_arch = "wasm32"), wasm_bindgen)]
pub enum Priority {
    High,
    Normal,
}

#[derive(Debug, Clone)]
#[cfg(any(
    feature = "xiaomi",
    feature = "huawei",
    feature = "fcm",
    feature = "client"
))]
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
    pub click_action: Option<&'a str>,
    pub body_loc_key: Option<&'a str>,
    pub body_loc_args: Option<&'a [String]>,
    pub title_loc_key: Option<&'a str>,
    pub title_loc_args: Option<&'a [String]>,
    pub channel_id: Option<&'a str>,
    pub image: Option<&'a str>,
    pub ticker: Option<&'a str>,
    pub visibility: Option<Visibility>,
    // xiaomi required
    pub package_name: Option<&'a str>,
    // huawei
    pub auto_clear: Option<i8>,
    // huawei and xiaomi
    pub foreground_show: Option<bool>,
    // xiaomi
    pub notify_id: Option<i32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
#[cfg(any(feature = "apns", feature = "client"))]
pub enum PushType {
    Alert,
    Voip,
}

#[derive(Debug, Serialize)]
#[cfg(any(feature = "apns", feature = "client"))]
pub struct IosExtra<'a> {
    pub push_type: PushType,
    pub topic: &'a str,
}

pub enum Transparent<'a> {
    Text(&'a str),
    Json(serde_json::Map<String, serde_json::Value>),
}

#[derive(Debug, Clone, Copy)]
pub enum Body<'a> {
    Transparent(&'a str),
    Notify { title: &'a str, body: &'a str },
}

#[derive(Debug, Clone)]
#[cfg(any(feature = "apns", feature = "client"))]
pub struct ApnsExtra<'a> {
    pub topic: &'a str,
    pub push_type: &'a apns::ApnsPushType,
}

#[derive(Debug, Clone)]
#[cfg(any(feature = "wecom", feature = "client"))]
pub enum WecomExtra<'a> {
    Markdown(bool),
    Text { url: &'a str, btntxt: &'a str },
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PushResult {
    pub request_id: String,
    pub code: String,
    pub reason: String,
    pub success: i64,
    pub failure: i64,
    pub invalid_tokens: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PushResults {
    pub success: i64,
    pub failure: i64,
    pub results: Vec<PushResult>,
}

#[derive(Debug, Clone)]
pub struct Message<'a> {
    pub tokens: &'a [&'a str],
    pub body: Body<'a>,
    #[cfg(any(feature = "xiaomi", feature = "huawei", feature = "fcm"))]
    pub android: Option<&'a AndroidExtra<'a>>,
    #[cfg(feature = "apns")]
    pub apns: Option<&'a ApnsExtra<'a>>,
    #[cfg(feature = "wecom")]
    pub wecom: Option<&'a WecomExtra<'a>>,
}
