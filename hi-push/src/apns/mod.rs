mod types;

pub use self::types::*;
use async_trait::async_trait;

use super::Error;

use reqwest::header;

pub struct Client {
    cli: reqwest::Client,
    production: bool,
}

impl Client {
    pub fn new(certs: &[u8], password: &str) -> Result<Self, Error> {
        let cert = reqwest::Identity::from_pkcs12_der(certs, password).unwrap();

        let cli = reqwest::Client::builder().identity(cert).build().unwrap();

        Ok(Self {
            cli,
            production: false,
        })
    }

    pub async fn _push<'b>(&self, notification: &'b Notification<'_>) -> Result<Response, Error> {
        let url = self.build_url(&notification.device_token);

        let mut headers = header::HeaderMap::new();

        headers.append(
            "apns-topic",
            header::HeaderValue::from_str(notification.topic).unwrap(),
        );

        if let Some(id) = notification.id {
            headers.append(
                "apns-id",
                header::HeaderValue::from_str(&id.to_string()).unwrap(),
            );
        }

        if let Some(expire) = notification.expiration {
            headers.append(
                "apns-expiration",
                header::HeaderValue::from_str(&expire.to_string()).unwrap(),
            );
        }

        if let Some(collapse_id) = &notification.collapse_id {
            headers.append(
                "apns-collapse-id",
                header::HeaderValue::from_str(collapse_id.as_str()).unwrap(),
            );
        }

        if let Some(priority) = notification.priority {
            headers.append(
                "apns-priority",
                header::HeaderValue::from_str(&priority.to_uint().to_string()).unwrap(),
            );
        }

        if let Some(push_type) = &notification.apns_push_type {
            headers.append(
                "apns-push-type",
                header::HeaderValue::from_str(push_type.as_str()).unwrap(),
            );
        }

        let request = ApnsRequest {
            aps: &notification.payload,
            custom: notification.custom.as_ref(),
        };
        let resp = self
            .cli
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .unwrap();

        let status_code = resp.status();

        let headers = resp.headers().clone();

        let apns_id = headers.get("apns-id").unwrap().clone();

        let apns_id = apns_id.to_str().unwrap();

        let mut resp = resp.json::<Response>().await.unwrap();

        resp.apns_id = apns_id.to_string();
        resp.status_code = status_code;
        resp.token = notification.device_token.to_string();

        Ok(resp)
    }

    //    pub async fn push<'b>(
    //        &self,
    //        notification: &'b [Notification<'_>],
    //    ) -> Result<BatchResponse, Error> {
    //        let mut resps = BatchResponse::default();
    //
    //        for notify in notification {
    //            let resp = self._push(notify).await;
    //            match resp {
    //                Ok(res) => {
    //                    if res.reason == ApiErrorReason::BadDeviceToken {
    //                        resps.responses.push(res);
    //                    }
    //                }
    //                Err(err) => {
    //                    resps.failure += 1
    //                }
    //            }
    //        }
    //        resps.failure = (notification.len() as i64 - resps.success) as i64;
    //        Ok(resps)
    //    }

    fn build_url(&self, device_token: &str) -> String {
        let root = if self.production {
            APN_URL_PRODUCTION
        } else {
            APN_URL_DEV
        };
        format!("{}/3/device/{}", root, device_token)
    }
}

#[async_trait]
impl<'a> super::Pusher<'a, Notification<'a>, Response> for Client {
    async fn push(&self, msg: &'a Notification) -> Result<Response, Error> {
        self._push(msg).await
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_cert() {}
}
