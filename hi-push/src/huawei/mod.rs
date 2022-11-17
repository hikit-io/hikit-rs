#[cfg(feature = "huawei-model")]
mod model;

#[cfg(feature = "huawei-model")]
pub use model::*;

#[cfg(feature = "huawei")]
pub use lib::*;

#[cfg(feature = "huawei")]
mod lib {
    pub use super::model::*;

    use http::StatusCode;
    use std::{sync::Arc, time::Duration};

    use serde::{Deserialize, Serialize};
    use serde_repr::Serialize_repr;

    use oauth2::TokenResponse;
    use oauth2::{
        basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
        reqwest::async_http_client,
        EmptyExtraTokenFields, RevocationErrorResponseType, StandardErrorResponse,
        StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse,
    };
    use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};
    use tokio::sync::RwLock;

    type Token = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

    type AuthClient = oauth2::Client<
        StandardErrorResponse<BasicErrorResponseType>,
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        BasicTokenType,
        StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardRevocableToken,
        StandardErrorResponse<RevocationErrorResponseType>,
    >;

    pub struct Client {
        client_id: String,
        auth: AuthClient,
        token: Arc<RwLock<Option<Token>>>,
        cli: reqwest::Client,
    }


    impl Client {
        const TOKEN_URL: &'static str = "https://oauth-login.cloud.huawei.com/oauth2/v3/token";

        pub async fn new(client_id: &str, client_secret: &str) -> Result<Client, crate::Error> {
            let auth = BasicClient::new(
                ClientId::new(client_id.to_string()),
                Some(ClientSecret::new(client_secret.to_string())),
                AuthUrl::new(Self::TOKEN_URL.to_string())
                    .map_err(|e| crate::RetryError::Auth(e.to_string()))?,
                Some(
                    TokenUrl::new(Self::TOKEN_URL.to_string())
                        .map_err(|e| crate::RetryError::Auth(e.to_string()))?,
                ),
            );

            let auth = auth.set_auth_type(oauth2::AuthType::RequestBody);

            let cli = reqwest::Client::builder()
                .build()
                .map_err(|e| crate::InnerError::Http(e.to_string()))?;

            let res = Client {
                auth,
                client_id: client_id.to_string(),
                token: Default::default(),
                cli,
            };

            res.request_token().await?;

            Ok(res)
        }

        /*
            request and set token
        */
        async fn request_token(&self) -> Result<Token, crate::Error> {
            let token = self
                .auth
                .exchange_client_credentials()
                .request_async(async_http_client)
                .await
                .map_err(|e| crate::RetryError::Auth(e.to_string()))?;
            self.set_token(token.clone()).await;

            Ok(token)
        }

        async fn set_token(&self, mut token: Token) {
            let expires_in =
                chrono::Utc::now().timestamp() as u64 + (token.expires_in().unwrap().as_secs());
            token.set_expires_in(Some(&Duration::from_secs(expires_in)));
            *(self.token.write().await) = Some(token);
        }

        /*
            valid token before pushing
        */
        fn valid_token(&self, token: &Token) -> bool {
            let expires = token.expires_in();
            if expires.is_none() {
                return false;
            }
            if expires.unwrap().as_secs() <= chrono::Utc::now().timestamp() as u64 {
                return false;
            }
            true
        }

        #[inline]
        fn build_push_url(&self) -> String {
            format!(
                "https://push-api.cloud.huawei.com/v1/{}/messages:send",
                self.client_id
            )
        }
    }

    #[async_trait::async_trait]
    impl<'b> crate::Pusher<'b, Message<'b>, Response> for Client {
        async fn push(&self, msg: &'b Message) -> Result<Response, crate::Error> {
            let token = self.token.clone();

            let token = token.read().await;

            let token = match token.clone() {
                Some(token) => token.clone(),
                None => match self.request_token().await {
                    Ok(token) => token,
                    Err(e) => return Err(crate::RetryError::Auth(e.to_string()).into()),
                },
            };

            if !self.valid_token(&token) {
                return Err(crate::RetryError::Auth("token expired or invalid".to_string()).into());
            }

            let resp = self
                .cli
                .post(self.build_push_url())
                .bearer_auth(token.access_token().secret())
                .json(msg)
                .send()
                .await?;

            let status = resp.status();

            match status {
                StatusCode::OK | StatusCode::BAD_REQUEST => {
                    let resp = resp.json::<SendResponse>().await?;
                    let invalid = resp.get_invalid_tokens();

                    let mut res = Response {
                        msg: resp.msg.clone(),
                        code: resp.code.clone(),
                        request_id: resp.request_id.clone(),
                        success: 0,
                        failure: 0,
                        illegal_tokens: vec![],
                    };
                    match resp.code {
                        Code::Success => {}
                        Code::PartFailedErr => {
                            res.success = invalid
                                .as_ref()
                                .map_or(msg.message.token.len() as i64, |e| e.success);
                            res.failure = invalid.as_ref().map_or(0, |e| e.failure);
                            res.illegal_tokens =
                                invalid.map_or(Default::default(), |e| e.illegal_tokens);
                        }
                        Code::ParameterError
                        | Code::TokenMustOne
                        | Code::MsgBodyError
                        | Code::TTLErr => {
                            return Err(crate::InnerError::InvalidParams(resp.msg).into());
                        }
                        Code::AuthFailedErr | Code::AuthTokenTimeoutErr => {
                            return Err(crate::RetryError::Auth(resp.msg).into());
                        }
                        Code::TokenInvalid => {
                            res.failure = msg.message.token.len() as i64;
                            res.illegal_tokens =
                                msg.message.token.iter().map(|e| e.to_string()).collect();
                        }
                        Code::Other(_) | Code::Common => {
                            return Err(crate::InnerError::Unknown(format!("{:?}", resp)).into());
                        }
                    }
                    Ok(res)
                }
                _ => match resp.error_for_status() {
                    Ok(_) => unreachable!(""),
                    Err(e) => Err(e)?,
                },
            }
        }
    }
}


#[cfg(feature = "huawei")]
#[cfg_attr(feature = "huawei", test)]
mod tests {
    use crate::Pusher;

    #[tokio::test]
    async fn test_push() {
        use crate::*;
        use super::*;

        let client_id = std::env::var("HW_CLIENT_ID").unwrap();
        let client_secret = std::env::var("HW_CLIENT_SECRET").unwrap();

        let hw = Client::new(&client_id, &client_secret).await.unwrap();
        let msg = Message {
            validate_only: false,
            message: InnerMessage {
                data: Some("hello"),
                notification: None,
                android: Some(AndroidConfig {
                    ..Default::default()
                }),
                token: vec![
                    "IQAAAACy0kYwAADWsJ-W5yOcL9booZrr1XdycVGvPWwWVrBG3AR838oq8gHM26Od6g_cxkQO_U1NbR720haQQ3VapXWyDMZyYj-MrSJeqUoq5k79Lw",
                    "1IQAAAACy0kYwAADWsJ-W5yOcL9booZrr1XdycVGvPWwWVrBG3AR838oq8gHM26Od6g_cxkQO_U1NbR720haQQ3VapXWyDMZyYj-MrSJeqUoq5k79Lw",
                ],
            },
        };
        let resp = hw.push(&msg).await;

        println!("{resp:?}");
    }
}
