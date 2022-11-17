use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "apns-model")]
use crate::apns;

#[wasm_bindgen]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterTokenResp {
    pub success: u64,
    pub failure: u64,
    failure_tokens: Vec<String>,
}

#[wasm_bindgen]
impl RegisterTokenResp {
    #[wasm_bindgen(setter, js_name = failureTokens)]
    pub fn set_failure_tokens(&mut self, failure_tokens: Vec<js_sys::JsString>) {
        self.failure_tokens = failure_tokens.iter().map(|e| e.into()).collect();
    }
    #[wasm_bindgen(getter, js_name = failureTokens)]
    pub fn failure_tokens(&self) -> Vec<js_sys::JsString> {
        self.failure_tokens.iter().map(|e| js_sys::JsString::from(e.as_str())).collect()
    }
}


#[wasm_bindgen]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterTokenParams {
    group: String,
    token: String,
    ch_id: String,
    #[serde(rename = "override")]
    _override: Option<bool>,
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

    #[wasm_bindgen(setter)]
    pub fn set_group(&mut self, group: String) {
        self.group = group;
    }
    #[wasm_bindgen(getter)]
    pub fn group(&mut self) -> String {
        self.group.clone()
    }

    #[wasm_bindgen(getter, js_name = "chId")]
    pub fn ch_id(&mut self) -> String {
        self.ch_id.clone()
    }
    #[wasm_bindgen(setter, js_name = "chId")]
    pub fn set_ch_id(&mut self, ch_id: js_sys::JsString) {
        self.ch_id = ch_id.into();
    }

    #[wasm_bindgen(getter)]
    pub fn token(&mut self) -> String {
        self.token.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_token(&mut self, token: js_sys::JsString) {
        self.token = token.into();
    }

    #[wasm_bindgen(setter, js_name = "override")]
    pub fn set_override(&mut self, _override: Option<bool>) {
        self._override = _override;
    }
    #[wasm_bindgen(getter, js_name = "override")]
    pub fn _override(&self) -> Option<bool> {
        self._override
    }
}


#[wasm_bindgen]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeTokenParams {
    group: String,
    token: String,
    ch_id: String,
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

    #[wasm_bindgen(setter)]
    pub fn set_group(&mut self, group: String) {
        self.group = group;
    }
    #[wasm_bindgen(getter)]
    pub fn group(&mut self) -> String {
        self.group.clone()
    }

    #[wasm_bindgen(getter, js_name = "chId")]
    pub fn ch_id(&mut self) -> String {
        self.ch_id.clone()
    }
    #[wasm_bindgen(setter, js_name = "chId")]
    pub fn set_ch_id(&mut self, ch_id: String) {
        self.ch_id = ch_id;
    }

    #[wasm_bindgen(getter)]
    pub fn token(&mut self) -> String {
        self.token.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }
}


#[wasm_bindgen]
#[derive(Deserialize)]
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
        self.failure_tokens.iter().map(|e| js_sys::JsString::from(e.as_str())).collect()
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
        self.channels.iter().map(|e| js_sys::JsString::from(e.as_str())).collect()
    }
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct Options {
    condition: Condition,
}

#[wasm_bindgen]
impl Options {
    #[wasm_bindgen(getter)]
    pub fn condition(&self) -> Condition {
        self.condition.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_condition(&mut self, condition: Condition) {
        self.condition = condition;
    }
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct Body {
    json: Option<serde_json::Map<String, serde_json::Value>>,
    text: Option<String>,
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
    #[wasm_bindgen(setter)]
    pub fn set_text(&mut self, text: Option<js_sys::JsString>) {
        self.text = text.map(|e| e.into());
    }
    #[wasm_bindgen(getter)]
    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct PushTransparentParams {
    groups: Vec<String>,
    channels: Vec<String>,
    #[serde(flatten)]
    body: Body,
    #[serde(flatten)]
    platform_extra: PlatformParams,
}


#[cfg(feature = "client")]
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "client", wasm_bindgen)]
pub struct WecomExtraText {
    url: String,
    btntxt: String,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "client", wasm_bindgen)]
pub struct WecomExtra {
    markdown: bool,
    text: WecomExtraText,
}

#[wasm_bindgen]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApnsExtra {
    topic: String,
    push_type: apns::ApnsPushType,
}


#[wasm_bindgen]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidExtra {
    pub collapse_key: Option<i64>,
    pub priority: Option<crate::Priority>,
    pub ttl: Option<i64>,
    title: Option<String>,
    body: Option<String>,
    icon: Option<String>,
    color: Option<String>,
    sound: Option<String>,
    tag: Option<String>,

    // huawei required
    click_action: Option<String>,
    body_loc_key: Option<String>,
    body_loc_args: Option<Vec<String>>,
    title_loc_key: Option<String>,
    title_loc_args: Option<Vec<String>>,
    channel_id: Option<String>,
    image: Option<String>,
    ticker: Option<String>,
    pub visibility: Option<crate::Visibility>,
    // xiaomi required
    package_name: Option<String>,
    // huawei
    pub auto_clear: Option<i8>,
    // huawei and xiaomi
    pub foreground_show: Option<bool>,
    // xiaomi
    pub notify_id: Option<i32>,
}

#[wasm_bindgen]
impl AndroidExtra {}


#[wasm_bindgen]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformParams {
    #[cfg(any(
    feature = "fcm",
    feature = "xiaomi",
    feature = "huawei",
    feature = "client"
    ))]
    android: Option<AndroidExtra>,
    #[cfg(any(feature = "apns", feature = "client"))]
    apns: Option<ApnsExtra>,
    #[cfg(any(feature = "wecom", feature = "client"))]
    wecom: Option<WecomExtra>,
}


#[cfg_attr(feature = "client", wasm_bindgen)]
pub struct WecomChannelConfig {
    client_id: String,
    client_secret: String,
    agentid: i64,
}

#[cfg_attr(feature = "client", wasm_bindgen)]
pub struct PublicChannel {
    wecom: Option<WecomChannelConfig>,
}
