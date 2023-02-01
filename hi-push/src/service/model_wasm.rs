use js_sys::JsString;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[allow(non_snake_case)]
#[cfg(feature = "apns-model")]
use crate::apns;

pub use super::common_model::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response<T = String> {
    pub code: Code,
    pub data: Option<T>,
    pub msg: String,
    pub errors: Option<Vec<String>>,
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

#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct ResponseError {
    pub code: Code,
    #[wasm_bindgen(getter_with_clone)]
    pub msg: String,
    errors: Vec<String>,
}

#[wasm_bindgen]
impl ResponseError {
    #[wasm_bindgen(setter)]
    pub fn set_errors(&mut self, errors: Vec<JsString>) {}

    #[wasm_bindgen(getter)]
    pub fn errors(&self) -> Vec<JsString> {
        self.errors
            .iter()
            .map(|e| e.clone().into())
            .collect::<Vec<_>>()
    }

    pub fn unknown(msg: &str) -> Self {
        Self {
            code: Code::Unknown,
            msg: msg.to_string(),
            errors: vec![],
        }
    }
}

impl From<reqwest::Error> for ResponseError {
    fn from(e: Error) -> Self {
        Self {
            code: Code::System,
            msg: e.to_string(),
            errors: vec![],
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct RegisterTokenResp {
    pub success: u64,
    pub failure: u64,
    failure_tokens: Vec<String>,
}

#[wasm_bindgen]
impl RegisterTokenResp {
    #[wasm_bindgen(setter, js_name = "failureTokens")]
    pub fn set_failure_tokens(&mut self, failure_tokens: Vec<js_sys::JsString>) {
        self.failure_tokens = failure_tokens.iter().map(|e| e.into()).collect();
    }
    #[wasm_bindgen(getter, js_name = "failureTokens")]
    pub fn failure_tokens(&self) -> Vec<js_sys::JsString> {
        self.failure_tokens
            .iter()
            .map(|e| js_sys::JsString::from(e.as_str()))
            .collect()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct RegisterTokenParams {
    pub group: String,
    pub token: String,
    pub ch_id: String,
    #[serde(rename = "override")]
    #[wasm_bindgen(js_name = "override")]
    pub _override: Option<bool>,
}

#[wasm_bindgen]
impl RegisterTokenParams {
    #[wasm_bindgen(constructor)]
    pub fn new(group: String, token: String, ch_id: String, _override: Option<bool>) -> Self {
        Self {
            group,
            token,
            ch_id,
            _override,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct RevokeTokenParams {
    pub group: String,
    pub token: String,
    #[wasm_bindgen(js_name = "chId")]
    pub ch_id: String,
}

#[wasm_bindgen]
impl RevokeTokenParams {
    #[wasm_bindgen(constructor)]
    pub fn new(group: String, token: String, ch_id: String) -> Self {
        Self {
            group,
            token,
            ch_id,
        }
    }
}

#[derive(Deserialize)]
#[wasm_bindgen]
#[serde(rename_all = "camelCase")]
pub struct RevokeTokenResp {
    pub success: u64,
    pub failure: u64,
    failure_tokens: Vec<String>,
}

#[wasm_bindgen]
impl RevokeTokenResp {
    #[wasm_bindgen(setter, js_name = "failureTokens")]
    pub fn set_failure_tokens(&mut self, failure_tokens: Vec<js_sys::JsString>) {
        self.failure_tokens = failure_tokens.iter().map(|e| e.into()).collect();
    }
    #[wasm_bindgen(getter, js_name = "failureTokens")]
    pub fn failure_tokens(&self) -> Vec<js_sys::JsString> {
        self.failure_tokens
            .iter()
            .map(|e| js_sys::JsString::from(e.as_str()))
            .collect()
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct Condition {
    channels: Vec<String>,
}

#[wasm_bindgen]
impl Condition {
    #[wasm_bindgen(setter, js_name = "channels")]
    pub fn set_channels(&mut self, channels: Vec<js_sys::JsString>) {
        self.channels = channels.iter().map(|e| e.into()).collect();
    }
    #[wasm_bindgen(getter, js_name = "channels")]
    pub fn channels(&self) -> Vec<js_sys::JsString> {
        self.channels
            .iter()
            .map(|e| js_sys::JsString::from(e.as_str()))
            .collect()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct Options {
    pub condition: Condition,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct Body {
    json: Option<serde_json::Map<String, serde_json::Value>>,
    pub text: Option<String>,
}

#[wasm_bindgen]
impl Body {
    #[wasm_bindgen(constructor)]
    pub fn new(text: Option<js_sys::JsString>) -> Self {
        Self {
            json: None,
            text: text.map(|e| e.into()),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct PushTransparentParams {
    groups: Vec<String>,
    channels: Vec<String>,
    #[serde(flatten)]
    pub body: Body,
    #[serde(flatten)]
    pub platform_extra: PlatformParams,
}

#[wasm_bindgen]
impl PushTransparentParams {
    #[wasm_bindgen(setter)]
    pub fn set_groups(&mut self, groups: Vec<JsString>) {
        self.groups = groups.iter().map(|e| e.into()).collect();
    }
    #[wasm_bindgen(getter)]
    pub fn groups(&self) -> Vec<JsString> {
        self.groups
            .iter()
            .map(|e| JsString::from(e.as_str()))
            .collect()
    }

    #[wasm_bindgen(setter)]
    pub fn set_channels(&mut self, channels: Vec<JsString>) {
        self.channels = channels.iter().map(|e| e.into()).collect();
    }
    #[wasm_bindgen(getter)]
    pub fn channels(&self) -> Vec<JsString> {
        self.channels
            .iter()
            .map(|e| JsString::from(e.as_str()))
            .collect()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct PushNotificationParams {
    groups: Vec<String>,
    channels: Vec<String>,
    pub title: String,
    pub body: String,
    pub wecom: WecomExtra,
}

#[wasm_bindgen]
impl PushNotificationParams {
    #[wasm_bindgen(setter)]
    pub fn set_groups(&mut self, groups: Vec<JsString>) {
        self.groups = groups.iter().map(|e| e.into()).collect();
    }
    #[wasm_bindgen(getter)]
    pub fn groups(&self) -> Vec<JsString> {
        self.groups
            .iter()
            .map(|e| JsString::from(e.as_str()))
            .collect()
    }

    #[wasm_bindgen(setter)]
    pub fn set_channels(&mut self, channels: Vec<JsString>) {
        self.channels = channels.iter().map(|e| e.into()).collect();
    }
    #[wasm_bindgen(getter)]
    pub fn channels(&self) -> Vec<JsString> {
        self.channels
            .iter()
            .map(|e| JsString::from(e.as_str()))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct WecomExtraText {
    pub url: String,
    pub btntxt: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct WecomExtra {
    pub markdown: bool,
    pub text: WecomExtraText,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct ApnsExtra {
    pub topic: String,
    push_type: apns::ApnsPushType,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
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
    body_loc_args: Option<Vec<String>>,
    pub title_loc_key: Option<String>,
    title_loc_args: Option<Vec<String>>,
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

#[wasm_bindgen]
impl AndroidExtra {
    #[wasm_bindgen(getter, js_name = "bodyLocArgs")]
    pub fn body_loc_args(&mut self) -> Vec<JsString> {
        if let Some(args) = &self.body_loc_args {
            return args.iter().map(|e| JsString::from(e.as_str())).collect();
        }
        Default::default()
    }
    #[wasm_bindgen(setter, js_name = "bodyLocArgs")]
    pub fn set_body_loc_args(&mut self, args: Vec<JsString>) {
        self.body_loc_args = Some(args.iter().map(|e| e.into()).collect());
    }

    #[wasm_bindgen(getter, js_name = "titleLocArgs")]
    pub fn title_loc_args(&mut self) -> Vec<JsString> {
        if let Some(args) = &self.title_loc_args {
            return args.iter().map(|e| JsString::from(e.as_str())).collect();
        }
        Default::default()
    }
    #[wasm_bindgen(setter, js_name = "titleLocArgs")]
    pub fn set_title_loc_args(&mut self, args: Vec<JsString>) {
        self.title_loc_args = Some(args.iter().map(|e| e.into()).collect());
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
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

#[derive(Debug, Serialize, Clone)]
#[cfg_attr(feature = "client", wasm_bindgen(getter_with_clone))]
pub struct WecomChannelConfig {
    pub client_id: String,
    pub client_secret: String,
    pub agentid: i64,
}

#[derive(Debug, Serialize, Clone)]
#[cfg_attr(feature = "client", wasm_bindgen(getter_with_clone))]
pub struct PublicChannel {
    pub wecom: Option<WecomChannelConfig>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct CreateApplicationParams {
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct DeleteApplicationParams {
    #[wasm_bindgen(js_name = "chId")]
    pub ch_id: String,
}
