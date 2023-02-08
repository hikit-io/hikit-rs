use async_trait::async_trait;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, TokenResponse, TokenUrl,
};
use reqwest::header::HeaderMap;
use serde::Deserialize;

pub struct Client {
    cli: BasicClient,
    http_cli: reqwest::Client,
}

#[derive(Debug, Deserialize)]
pub struct UserResp {
    pub id: i64,
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct OrgResp {
    pub id: i64,
    pub login: String,
}

impl Client {
    pub fn new(client_id: &str, client_secret: &str, redirect_url: &str) -> Self {
        let cli = BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap(),
            Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap()),
        );

        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", "hikit_auth_service".parse().unwrap());
        let http_cli = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        Self { cli, http_cli }
    }

    pub async fn login(&self, code: &str) -> super::Result<String> {
        Ok(self
            .cli
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(async_http_client)
            .await
            .map(|resp| resp.access_token().secret().to_string())
            .map_err(|e| e.to_string())?)
    }

    pub async fn user(&self, access_token: &str) -> super::Result<UserResp> {
        Ok(self
            .http_cli
            .get("https://api.github.com/user")
            .bearer_auth(&access_token)
            .send()
            .await?
            .json::<UserResp>()
            .await?)
    }
    pub async fn orgs(&self, access_token: &str) -> super::Result<Vec<OrgResp>> {
        Ok(self
            .http_cli
            .get("https://api.github.com/user/orgs")
            .bearer_auth(&access_token)
            .send()
            .await?
            .json::<Vec<OrgResp>>()
            .await?)
    }
}

#[async_trait]
impl super::Login<UserResp> for Client {
    async fn userinfo(&self, code: &str) -> crate::Result<UserResp> {
        let at = self.login(code).await?;
        let user = self.user(&at).await?;
        Ok(user)
    }
}
