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
pub struct User {
    pub id: i64,
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct Organization {
    pub id: i64,
    pub login: String,
}

impl Client {
    pub fn new(client_id: &str, client_secret: &str) -> Self {
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
            .map_err(|e| super::Error(e.to_string()))?)
    }

    pub async fn user(&self, access_token: &str) -> super::Result<User> {
        Ok(self
            .http_cli
            .get("https://api.github.com/user")
            .bearer_auth(&access_token)
            .send()
            .await?
            .json::<User>()
            .await?)
    }
    pub async fn orgs(&self, access_token: &str) -> super::Result<Vec<Organization>> {
        Ok(self
            .http_cli
            .get("https://api.github.com/user/orgs")
            .bearer_auth(&access_token)
            .send()
            .await?
            .json::<Vec<Organization>>()
            .await?)
    }
}

#[async_trait]
impl super::Profile for Client {
    async fn userinfo(&self, code: &str) -> crate::Result<super::Userinfo> {
        let at = self.login(code).await?;
        let user = self.user(&at).await?;
        let orgs = self.orgs(&at).await?;
        Ok(super::Userinfo{
            unique_id: user.id.to_string(),
            name: user.login,
            email: None,
            organization: Some(orgs.into_iter().map(|e|super::Organization{
                unique_id: e.id.to_string(),
                name: e.login,
            }).collect())
        })
    }
}
