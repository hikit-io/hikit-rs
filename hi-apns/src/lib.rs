#![deny(warnings)]

mod types;
pub use self::types::*;

mod error;
use self::error::*;

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use curl::easy::{Easy2, Handler, HttpVersion, List, WriteError};
use failure::Error;
use reqwest::header;
use uuid::Uuid;

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

    pub async fn send(&self, notification: Notification<'_>) -> Result<Response, SendError> {
        let url = self.build_url(&notification.device_token);

        let mut headers = header::HeaderMap::new();

        headers.append(
            "apns-topic",
            header::HeaderValue::from_str(&notification.topic).unwrap(),
        );

        if let Some(id) = notification.id {
            headers.append(
                "apns-id",
                header::HeaderValue::from_str(&id.to_string()).unwrap(),
            );
        }

        if let Some(expire) = notification.expiration {
            headers.append(
                "apns-expiration                ",
                header::HeaderValue::from_str(&expire.to_string()).unwrap(),
            );
        }

        if let Some(collapse_id) = notification.collapse_id {
            headers.append(
                "apns-collapse-id                ",
                header::HeaderValue::from_str(collapse_id.as_str()).unwrap(),
            );
        }

        if let Some(priority) = notification.priority {
            headers.append(
                "apns-priority",
                header::HeaderValue::from_str(&priority.to_uint().to_string()).unwrap(),
            );
        }

        if let Some(push_type) = notification.apns_push_type {
            headers.append(
                "apns-push-type",
                header::HeaderValue::from_str(push_type.as_str()).unwrap(),
            );
        }

        let request = ApnsRequest {
            aps: notification.payload,
        };

        let mut resp = self
            .cli
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .unwrap();

        let status_code = resp.status();

        let headers = std::mem::take(resp.headers_mut());

        let apns_id = headers.get("apns-id").unwrap().clone();

        let apns_id = apns_id.to_str().unwrap();

        let mut resp = resp.json::<Response>().await.unwrap();

        resp.apns_id = apns_id.to_string();
        resp.status_code = status_code;

        Ok(resp)
    }

    fn build_url(&self, device_token: &str) -> String {
        let root = if self.production {
            APN_URL_PRODUCTION
        } else {
            APN_URL_DEV
        };
        format!("{}/3/device/{}", root, device_token)
    }
}

/// Writer used by curl.
struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

#[derive(Clone, Debug)]
pub struct ProviderCertificate {
    pub p12_path: PathBuf,
    pub passphrase: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Auth {
    ProviderCertificate(ProviderCertificate),
}

impl Auth {
    fn as_cert(&self) -> &ProviderCertificate {
        match self {
            &Auth::ProviderCertificate(ref c) => c,
        }
    }
}

pub struct ApnsSync {
    production: bool,
    verbose: bool,
    delivery_disabled: bool,
    auth: Auth,
    easy: RefCell<Easy2<Collector>>,
}

impl ApnsSync {
    pub fn new(auth: Auth) -> Result<Self, Error> {
        let mut easy = Easy2::new(Collector(Vec::new()));

        easy.http_version(HttpVersion::V2)?;
        // easy.connect_only(true)?;
        // easy.url(APN_URL_PRODUCTION)?;

        // Configure curl for client certificate.
        {
            let cert = auth.as_cert();

            easy.ssl_cert(&cert.p12_path)?;
            if let Some(ref pw) = cert.passphrase.as_ref() {
                easy.key_password(&pw)?;
            }
        }

        let apns = ApnsSync {
            production: true,
            verbose: false,
            delivery_disabled: false,
            auth,
            easy: RefCell::new(easy),
        };
        Ok(apns)
    }

    pub fn with_certificate<P: AsRef<Path>>(
        path: P,
        passphrase: Option<String>,
    ) -> Result<ApnsSync, Error> {
        Self::new(Auth::ProviderCertificate(ProviderCertificate {
            p12_path: path.as_ref().to_path_buf(),
            passphrase,
        }))
    }

    /// Enable/disable verbose debug logging to stderr.
    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    /// Set API endpoint to use (production or development sandbox).
    pub fn set_production(&mut self, production: bool) {
        self.production = production;
    }

    /// *ATTENTION*: This completely disables actual communication with the
    /// APNS api.
    ///
    /// No connection will be established.
    ///
    /// Useful for integration tests in a larger application when nothing should
    /// actually be sent.
    pub fn disable_delivery_for_testing(&mut self) {
        self.delivery_disabled = true;
    }

    /// Build the url for a device token.
    fn build_url(&self, device_token: &str) -> String {
        let root = if self.production {
            APN_URL_PRODUCTION
        } else {
            APN_URL_DEV
        };
        format!("{}/3/device/{}", root, device_token)
    }

    /// Send a notification.
    /// Returns the UUID (either the configured one, or the one returned by the
    /// api).
    pub fn send(&self, notification: Notification) -> Result<Uuid, SendError> {
        let n = notification;

        // Just always generate a uuid client side for simplicity.
        let id = n.id.unwrap_or(Uuid::new_v4());

        if self.delivery_disabled {
            return Ok(id);
        }

        let url = self.build_url(&n.device_token);

        // Add headers.

        let mut headers = List::new();

        // NOTE: if an option which requires a header is not set,
        // the header is still added, but with an empty value,
        // which instructs curl to drop the header.
        // Otherwhise, headers from previous runs would stick around.

        headers.append(&format!("apns-id:{}", id.to_string(),))?;
        headers.append(&format!(
            "apns-expiration:{}",
            n.expiration
                .map(|x| x.to_string())
                .unwrap_or("".to_string())
        ))?;
        headers.append(&format!(
            "apns-priority:{}",
            n.priority
                .map(|x| x.to_uint().to_string())
                .unwrap_or("".to_string())
        ))?;
        headers.append(&format!("apns-topic:{}", n.topic))?;
        headers.append(&format!(
            "apns-collapse-id:{}",
            n.collapse_id
                .map(|x| x.as_str().to_string())
                .unwrap_or("".to_string())
        ))?;

        let request = ApnsRequest { aps: n.payload };
        let raw_request = serde_json::to_vec(&request)?;

        let mut easy = self.easy.borrow_mut();

        match &self.auth {
            _ => {}
        }

        easy.verbose(self.verbose)?;
        easy.http_headers(headers)?;
        easy.post(true)?;
        easy.post_fields_copy(&raw_request)?;
        easy.url(&url)?;
        easy.perform()?;

        let status = easy.response_code()?;
        if status != 200 {
            // Request failed.
            // Read json response with the error.
            let response_data = easy.get_ref();
            let reason = ErrorResponse::parse_payload(&response_data.0);
            Err(ApiError { status, reason }.into())
        } else {
            Ok(id)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env::var;

    #[test]
    fn test_cert() {
        let cert_path = var("APNS_CERT_PATH").unwrap();
        let cert_pw = Some(var("APNS_CERT_PW").unwrap());
        let topic = var("APNS_TOPIC").unwrap();
        let token = var("APNS_DEVICE_TOKEN").unwrap();

        let mut apns = ApnsSync::with_certificate(cert_path, cert_pw).unwrap();
        apns.set_verbose(true);
        let n = NotificationBuilder::new(&topic, &token)
            .title("title")
            .build();
        apns.send(n).unwrap();
    }
}
