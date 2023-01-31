#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::service::model::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Client {
    cli: reqwest::Client,
    client_id: String,
    client_secret: String,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Client {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(endpoint: &str, client_id: String, client_secret: String) -> Self {
        let cli = reqwest::Client::new();
        Self {
            cli,
            client_id,
            client_secret,
        }
    }
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "registerToken"))]
    pub fn register_token(&self, msg: RegisterTokenParams) -> RegisterTokenResp {
        serde_json::from_str("").unwrap()
    }
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "revokeToken"))]
    pub fn revoke_token(&self, msg: RevokeTokenParams) -> RevokeTokenResp {
        serde_json::from_str("").unwrap()
    }
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "pushTransparentMessage"))]
    pub fn push_transparent_message(&self, msg: PushTransparentParams) -> String {
        "".to_string()
    }
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "pushNotificationMessage"))]
    pub fn push_notification_message(&self, msg: PushNotificationParams) -> String {
        "".to_string()
    }
}
