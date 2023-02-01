#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "client"))]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Client {
    cli: reqwest::Client,
    client_id: String,
    client_secret: String,
    endpoint: String,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Client {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(endpoint: &str, client_id: &str, client_secret: &str) -> Self {
        let cli = reqwest::Client::new();
        Self {
            cli,
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            endpoint: endpoint.to_string(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub async fn debug(&self) -> String {
        "debug".to_string()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "registerToken"))]
    pub async fn register_token(
        &self,
        params: RegisterTokenParams,
    ) -> Result<RegisterTokenResp, ResponseError> {
        let resp = self
            .cli
            .post(format!("{}", self.endpoint))
            .json(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?;
        let n_js: Response<RegisterTokenResp> = resp.json().await?;
        if n_js.code != Code::Ok {
            Err(n_js.into())
        } else {
            Ok(n_js.data.ok_or(ResponseError::unknown(""))?)
        }
    }
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "revokeToken"))]
    pub async fn revoke_token(&self, params: RevokeTokenParams) -> RevokeTokenResp {
        self.cli
            .post("/")
            .json(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
    #[cfg_attr(
        target_arch = "wasm32",
        wasm_bindgen(js_name = "pushTransparentMessage")
    )]
    pub async fn push_transparent_message(&self, params: PushTransparentParams) -> String {
        self.cli
            .post("/")
            .json(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
    #[cfg_attr(
        target_arch = "wasm32",
        wasm_bindgen(js_name = "pushNotificationMessage")
    )]
    pub async fn push_notification_message(&self, params: PushNotificationParams) -> String {
        self.cli
            .post("/")
            .json(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "createApplication"))]
    pub async fn create_application(&self, params: CreateApplicationParams) -> String {
        self.cli
            .post("/")
            .json(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = "deleteApplication"))]
    pub async fn delete_application(&self, params: DeleteApplicationParams) -> String {
        self.cli
            .delete("/")
            .json(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}
