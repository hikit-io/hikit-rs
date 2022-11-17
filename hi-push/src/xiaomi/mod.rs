#[cfg(feature = "xiaomi-model")]
mod model;

#[cfg(feature = "xiaomi-model")]
pub use model::*;


#[cfg(feature = "xiaomi")]
pub use lib::*;

#[cfg(feature = "xiaomi")]
mod lib {
    use super::model::*;
    use std::ops::Deref;

    /**
     * https://dev.mi.com/console/doc/detail?pId=2086
     */
    use reqwest::header;
    use serde::{Deserialize, Serialize};
    use serde_repr::{Deserialize_repr, Serialize_repr};

    pub struct Config<'a> {
        pub client_id: &'a str,
        pub client_secret: &'a str,
        pub project_id: &'a str,
    }

    pub struct Client {
        cli: reqwest::Client,
    }

    impl<'a> Client {
        const PUSH_URL: &'static str = "https://api.xmpush.xiaomi.com/v4/message/regid";

        pub fn new(conf: &'a Config<'a>) -> Result<Self, crate::Error> {
            let mut default_header = header::HeaderMap::new();

            unsafe {
                default_header.insert(
                    header::AUTHORIZATION,
                    header::HeaderValue::from_str(&format!("key={}", conf.client_secret))
                        .unwrap_unchecked(), // @unsafe
                );
            }

            let cli = reqwest::Client::builder()
                .default_headers(default_header)
                .build()
                .map_err(|e| crate::InnerError::Http(e.to_string()))?;

            Ok(Client { cli })
        }
    }

    #[async_trait::async_trait]
    impl<'b> crate::Pusher<'b, Message<'b>, Response> for Client {
        const TOKEN_LIMIT: usize = 1000;

        async fn push(&self, msg: &'b Message) -> Result<Response, crate::Error> {
            let resp = self
                .cli
                .post(Self::PUSH_URL)
                .form(msg)
                .send()
                .await?
                .error_for_status()?;

            match resp.json::<Response>().await {
                Ok(resp) => match resp.code {
                    0 => Ok(resp),
                    21301 => Err(crate::RetryError::Auth(format!("{:?}", resp)).into()),
                    200001 => Err(crate::InnerError::MessageLimit.into()),
                    200002 => Err(crate::RetryError::QPS.into()),
                    _ => Err(crate::InnerError::Response(format!("{:?}", resp)).into()),
                },
                Err(e) => Err(crate::InnerError::Response(e.to_string()).into()),
            }
        }
    }
}


#[cfg(feature = "xiaomi")]
#[cfg_attr(feature = "xiaomi", test)]
mod tests {
    use crate::Pusher;

    #[tokio::test]
    async fn test_client() {
        use crate::*;

        let client_id = std::env::var("MI_CLIENT_ID").unwrap();
        let client_secret = std::env::var("MI_CLIENT_SECRET").unwrap();
        let project_id = std::env::var("MI_PROJECT_ID").unwrap();

        let cli = Client::new(&Config {
            client_id: &client_id,
            client_secret: &client_secret,
            project_id: &project_id,
        })
            .unwrap();

        let mut msg = Message::default();
        msg.registration_id = Some(
            "8whvC7gdG2QzNRZHUUPDQQ01laI9ZavQ/HbDTvEbHG2/XrY2Jj02nOAgZZx3T2Xw,12341".to_string(),
        );
        msg.payload = Some("a=123");
        msg.extra = Extra {
            callback: "https://callback".into(),
            notify_foreground: Bool::True.into(),
            ..Default::default()
        }
            .into();
        let resp = cli.push(&msg).await;

        println!("{resp:?}");
    }
}
