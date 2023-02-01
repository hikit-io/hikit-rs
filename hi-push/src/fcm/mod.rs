#[cfg(feature = "fcm")]
pub use lib::*;
#[cfg(feature = "fcm-model")]
pub use model::*;

#[cfg(feature = "fcm-model")]
mod model;

#[cfg(feature = "fcm")]
mod lib {
    use std::str::FromStr;

    use async_trait::async_trait;
    use bytecodec::{
        bytes::{BytesEncoder, RemainingBytesDecoder},
        io::{IoDecodeExt, IoEncodeExt},
        Encode,
    };
    use hi_hyper_multipart as multipart;
    use http::Uri;
    use httpcodec::{
        BodyDecoder, BodyEncoder, HeaderField, HttpVersion, Method, RequestEncoder, RequestTarget,
        ResponseDecoder,
    };
    use hyper::client::connect::dns::GaiResolver;
    use hyper::client::HttpConnector;
    use hyper_socks2::SocksConnector;
    use hyper_tls::HttpsConnector;
    use reqwest::Proxy;
    use serde::Deserialize;
    use yup_oauth2 as oauth2;
    use yup_oauth2::AccessToken;

    use super::model::*;

    pub type Config = yup_oauth2::ServiceAccountKey;

    type ProxyAuthClient = oauth2::authenticator::Authenticator<
        HttpsConnector<SocksConnector<HttpConnector<GaiResolver>>>,
    >;
    type CommonAuthClient =
        oauth2::authenticator::Authenticator<HttpsConnector<HttpConnector<GaiResolver>>>;

    enum InnerAuthClient {
        Common(CommonAuthClient),
        Proxy(ProxyAuthClient),
    }

    pub struct Client {
        http: reqwest::Client,
        auth: InnerAuthClient,
        project_id: String,
        conf: Config,
    }

    pub struct ProxyConfig<'a> {
        pub addr: &'a str,
        pub user: Option<&'a str>,
        pub pass: Option<&'a str>,
    }

    impl Client {
        pub async fn new<'a>(conf: Config) -> Result<Self, crate::Error> {
            let project_id = conf.project_id.clone().unwrap();

            let mut connector = HttpConnector::new();
            connector.enforce_http(false);

            let conn = HttpsConnector::new_with_connector(connector);
            let http = hyper::Client::builder().build(conn);
            let auth = oauth2::ServiceAccountAuthenticator::builder(conf.clone())
                .hyper_client(http.clone())
                .build()
                .await
                .map_err(|e| crate::RetryError::Auth(e.to_string()))?;

            Ok(Self {
                http: reqwest::Client::builder().build().unwrap(),
                auth: InnerAuthClient::Common(auth),
                conf,
                project_id,
            })
        }
    }

    impl Client {
        const DEFAULT_MESSAGING_ENDPOINT: &'static str = "https://fcm.googleapis.com/v1";
        const DEFAULT_BATCH_ENDPOINT: &'static str = "https://fcm.googleapis.com/batch";

        #[inline]
        fn build_single_url(&self) -> String {
            format!(
                "{}/projects/{}/messages:send",
                Self::DEFAULT_MESSAGING_ENDPOINT,
                self.project_id
            )
        }

        /// Returns the build batch url of this [`Client`].
        #[inline]
        fn build_batch_url(&self) -> String {
            format!("{}", Self::DEFAULT_BATCH_ENDPOINT)
        }

        pub async fn with_proxy<'f>(&'f mut self, config: ProxyConfig<'f>) -> &'f mut Self {
            let mut connector = HttpConnector::new();
            connector.enforce_http(false);

            let auth = config.user.map_or(None, |e| {
                Some(hyper_socks2::Auth::new(e, config.pass.unwrap_or_default()))
            });
            let conn = SocksConnector {
                proxy_addr: Uri::from_str(config.addr).unwrap(), // scheme is required by HttpConnector
                auth,
                connector,
            }
            .with_tls()
            .unwrap();
            let cli = hyper::Client::builder().build(conn);
            let auth = oauth2::ServiceAccountAuthenticator::builder(self.conf.clone())
                .hyper_client(cli.clone())
                .build()
                .await
                .map_err(|e| crate::RetryError::Auth(e.to_string()))
                .unwrap();

            let mut proxy = Proxy::all(config.addr).unwrap();
            if let Some(user) = config.user {
                proxy = proxy.basic_auth(user, config.pass.unwrap_or_default());
            }

            self.http = reqwest::Client::builder().proxy(proxy).build().unwrap();
            self.auth = InnerAuthClient::Proxy(auth);
            self
        }

        pub async fn multicast_send<'b>(
            &self,
            msg: &MulticastMessage<'b>,
        ) -> Result<BatchResponse, crate::Error> {
            let mut form = multipart::Form::new();

            for (index, &token) in msg.tokens.iter().enumerate() {
                let mut encoder = RequestEncoder::new(BodyEncoder::new(BytesEncoder::new()));

                let text = serde_json::to_string(&SendMessageRequest {
                    message: Some(Message {
                        android: msg.android.clone(),
                        apns: msg.apns.clone(),
                        condition: None,
                        data: msg.data.clone(),
                        fcm_options: None,
                        name: None,
                        notification: msg.notification.clone(),
                        token: Some(token.to_string()),
                        topic: None,
                        webpush: msg.webpush.clone(),
                    }),
                    validate_only: None,
                })
                .unwrap();

                let mut buf = Vec::new();
                let mut req = httpcodec::Request::new(
                    Method::new("POST").unwrap(),
                    RequestTarget::new(&*self.build_single_url()).unwrap(),
                    HttpVersion::V1_1,
                    text,
                );

                req.header_mut()
                    .add_field(HeaderField::new("Content-Type", "application/json").unwrap());
                req.header_mut()
                    .add_field(HeaderField::new("User-Agent", "").unwrap());

                encoder.start_encoding(req).unwrap();

                encoder.encode_all(&mut buf).unwrap();

                let length = buf.len();

                let mut part = multipart::Part::text(String::from_utf8(buf).unwrap());

                let headers = part.headers_mut();

                headers.insert("Content-Length", length.to_string().parse().unwrap());
                headers.insert("Content-Type", "application/http".parse().unwrap());
                headers.insert("Content-Id", (index + 1).to_string().parse().unwrap());
                headers.insert("Content-Transfer-Encoding", "binary".parse().unwrap());

                form = form.part(index.to_string(), part);
            }

            let token = self.get_token().await?;

            let boundary = form.boundary().to_string();

            let url = self.build_batch_url();

            let mut resp = self
                .http
                .post(url)
                .header(
                    "Content-Type",
                    format!("multipart/mixed; boundary={}", boundary),
                )
                .bearer_auth(token.as_str())
                .body(form.stream())
                .send()
                .await
                .unwrap();
            let headers = std::mem::take(resp.headers_mut());

            let ct = headers
                .get("Content-Type")
                .clone()
                .map_or("", |e| e.to_str().unwrap());
            let boundary = ct.split("=").collect::<Vec<_>>().pop().unwrap_or_default();

            let mut mr = multer::Multipart::new(resp.bytes_stream(), boundary);

            let mut decoder = ResponseDecoder::<BodyDecoder<RemainingBytesDecoder>>::default();

            let mut return_resp = BatchResponse {
                success_count: 0,
                failure_count: 0,
                responses: vec![],
            };
            while let Some(field) = mr.next_field().await.unwrap() {
                let index = field.index();
                let text = field.text().await.unwrap();
                let res = decoder.decode_exact(text.as_bytes()).unwrap();

                let (_res, body) = res.take_body();

                let resp = serde_json::from_slice::<FcmResponse>(body.as_slice()).unwrap();

                match resp {
                    FcmResponse::Ok { name } => {
                        return_resp.success_count += 1;
                        return_resp
                            .responses
                            .push(SendResponse::Ok { message_id: name });
                    }
                    FcmResponse::Error { mut error } => {
                        return_resp.failure_count += 1;
                        let detail = error.details.pop().unwrap();
                        return_resp.responses.push(SendResponse::Error {
                            token: msg.tokens.get(index).unwrap().to_string(),
                            error: detail.error_code,
                        });
                    }
                }
            }
            Ok(return_resp)
        }

        async fn get_token(&self) -> Result<AccessToken, crate::Error> {
            match &self.auth {
                InnerAuthClient::Common(cli) => cli
                    .token(&["https://www.googleapis.com/auth/firebase.messaging"])
                    .await
                    .map_err(|e| crate::RetryError::Auth(e.to_string()).into()),
                InnerAuthClient::Proxy(cli) => cli
                    .token(&["https://www.googleapis.com/auth/firebase.messaging"])
                    .await
                    .map_err(|e| crate::RetryError::Auth(e.to_string()).into()),
            }
        }
    }

    #[derive(Deserialize)]
    pub enum SendResponse {
        Ok { message_id: String },
        Error { token: String, error: String },
    }

    #[derive(Deserialize)]
    pub struct BatchResponse {
        pub success_count: i64,
        pub failure_count: i64,
        pub responses: Vec<SendResponse>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum FcmResponse {
        Ok { name: String },
        Error { error: FcmError },
    }

    #[derive(Deserialize, Debug)]
    struct FcmErrorItem {
        #[serde(rename = "@type")]
        _type: String,
        #[serde(rename = "errorCode")]
        error_code: String,
    }

    #[derive(Deserialize, Debug)]
    struct FcmError {
        details: Vec<FcmErrorItem>,
    }

    #[async_trait]
    impl<'b> crate::Pusher<'b, MulticastMessage<'b>, BatchResponse> for Client {
        const TOKEN_LIMIT: usize = 1000;

        async fn push(&self, msg: &'b MulticastMessage) -> Result<BatchResponse, crate::Error> {
            self.multicast_send(msg).await
        }
    }
}

#[cfg(all(feature = "fcm", test))]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[tokio::test]
    async fn test_fcm() {
        let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap();
        let client_email = std::env::var("GOOGLE_CLIENT_EMAIL").unwrap();
        let private_id = std::env::var("GOOGLE_PRIVATE_ID").unwrap();
        let private_key = std::env::var("GOOGLE_PRIVATE_KEY").unwrap();
        let project_id = std::env::var("GOOGLE_PROJECT_ID").unwrap();

        println!("{}", private_key);

        let mut fcm = Client::new(Config {
            key_type: "service_account".to_string().into(),
            client_id: client_id.into(),
            private_key_id: private_id.into(),
            private_key,
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string().into(),
            project_id: project_id.into(),
            client_email,
            auth_provider_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".to_string()),
            client_x509_cert_url: Some("https://www.googleapis.com/robot/v1/metadata/x509/firebase-adminsdk-vle32%40avcf-7ea7e.iam.gserviceaccount.com".to_string()),
        })
            .await.unwrap();

        fcm.with_proxy(ProxyConfig {
            addr: "socks5://127.0.0.1:7890",
            user: None,
            pass: None,
        })
        .await;

        let res = fcm
            .multicast_send(&MulticastMessage {
                tokens: vec![
                    "fn-3aZfYyceipSqyB-iigS:APA91bEYAGIVMeDmrvZqV5T8C_5UUCrb9xlvupRuKOyHgHDJnYkuwnKfOPCoQKBIQ4IhEJdNPBlaTapVG-iBAYPZ8GegROoeQTetlvmmKPBQrH9hrVRTTaOW69qBm7ZoDy1ewPGqD5RC",
                    "fn-3aZfYyceipSqyB-iigS:APA91bEYAGIVMeDmrvZqV5T8C_5UUCrb9xlvupRuKOyHgHDJnYkuwnKfOPCoQKBIQ4IhEJdNPBlaTapVG-iBAYPZ8GegROoeQTetlvmmKPBQrH9hrVRTTaOW69qBm7ZoDy1ewPGqD5RA",
                ],
                data: Some(HashMap::new()),
                ..Default::default()
            })
            .await
            .unwrap();
        println!("{:?}", res.success_count);
    }
}
