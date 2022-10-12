use std::collections::HashMap;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

/// The reason for a failure returned by the APN api.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ApiErrorReason {
    BadCollapseId,
    BadDeviceToken,
    BadExpirationDate,
    BadMessageId,
    BadPriority,
    BadTopic,
    DeviceTokenNotForTopic,
    DuplicateHeaders,
    IdleTimeout,
    MissingDeviceToken,
    MissingTopic,
    PayloadEmpty,
    TopicDisallowed,
    BadCertificate,
    BadCertificateEnvironment,
    ExpiredProviderToken,
    Forbidden,
    InvalidProviderToken,
    MissingProviderToken,
    BadPath,
    MethodNotAllowed,
    Unregistered,
    PayloadTooLarge,
    TooManyProviderTokenUpdates,
    TooManyRequests,
    InternalServerError,
    ServiceUnavailable,
    Shutdown,
    Other(String),
}

impl ApiErrorReason {
    fn to_str(&self) -> &str {
        use self::ApiErrorReason::*;
        match self {
            &BadCollapseId => "BadCollapseId",
            &BadDeviceToken => "BadDeviceToken",
            &BadExpirationDate => "BadExpirationDate",
            &BadMessageId => "BadMessageId",
            &BadPriority => "BadPriority",
            &BadTopic => "BadTopic",
            &DeviceTokenNotForTopic => "DeviceTokenNotForTopic",
            &DuplicateHeaders => "DuplicateHeaders",
            &IdleTimeout => "IdleTimeout",
            &MissingDeviceToken => "MissingDeviceToken",
            &MissingTopic => "MissingTopic",
            &PayloadEmpty => "PayloadEmpty",
            &TopicDisallowed => "TopicDisallowed",
            &BadCertificate => "BadCertificate",
            &BadCertificateEnvironment => "BadCertificateEnvironment",
            &ExpiredProviderToken => "ExpiredProviderToken",
            &Forbidden => "Forbidden",
            &InvalidProviderToken => "InvalidProviderToken",
            &MissingProviderToken => "MissingProviderToken",
            &BadPath => "BadPath",
            &MethodNotAllowed => "MethodNotAllowed",
            &Unregistered => "Unregistered",
            &PayloadTooLarge => "PayloadTooLarge",
            &TooManyProviderTokenUpdates => "TooManyProviderTokenUpdates",
            &TooManyRequests => "TooManyRequests",
            &InternalServerError => "InternalServerError",
            &ServiceUnavailable => "ServiceUnavailable",
            &Shutdown => "Shutdown",
            &Other(ref val) => val,
        }
    }
}

/// APNS production endpoint.
pub static APN_URL_PRODUCTION: &'static str = "https://api.push.apple.com";

/// APNS development endpoint.
pub static APN_URL_DEV: &'static str = "https://api.sandbox.push.apple.com";

/// Notification priority.
/// See APNS documentation for the effects.
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Priority {
    Low = 1,
    // 5
    Middle = 5,
    High = 10, // 10
}

impl Priority {
    /// Convert Priority to it's numeric value.
    pub fn to_uint(self) -> u8 {
        self as u8
    }
}

#[derive(Debug)]
pub struct CollapseIdTooLongError;

/// Wrapper type for collapse ids.
/// It may be an arbitrary string, but is limited in length to at most 63 bytes.
#[derive(Serialize, Clone, Debug)]
pub struct CollapseId<'a>(&'a str);

impl<'a> CollapseId<'a> {
    /// Construct a new collapse id.
    /// Returns an error if id exceeds the maximum length of 64 bytes.
    pub fn new(value: &'a str) -> Result<Self, CollapseIdTooLongError> {
        // CollapseID must be at most 64 bytes long.
        if value.len() > 64 {
            Err(CollapseIdTooLongError)
        } else {
            Ok(CollapseId(value))
        }
    }

    /// Get id as a raw str.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ApnsPushType {
    Alert,
    Background,
    Voip,
    Location,
    Complication,
    Fileprovider,
    Mdm,
}

impl ApnsPushType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApnsPushType::Alert => "alert",
            ApnsPushType::Background => "background",
            ApnsPushType::Voip => "voip",
            ApnsPushType::Location => "location",
            ApnsPushType::Complication => "complication",
            ApnsPushType::Fileprovider => "fileprovider",
            ApnsPushType::Mdm => "mdm",
        }
    }
}

/// Alert content for a notification.
///
/// See the official documentation for details:
/// https://developer.apple.com/library/content/documentation/NetworkingInternet/Conceptual/RemoteNotificationsPG/PayloadKeyReference.html
#[derive(Serialize, Default, Clone, Debug)]
pub struct AlertPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<&'a str>,
    #[serde(rename = "title-loc-key", skip_serializing_if = "Option::is_none")]
    pub title_loc_key: Option<&'a str>,
    #[serde(rename = "title-loc-args", skip_serializing_if = "Option::is_none")]
    pub title_loc_args: Option<Vec<String>>,
    #[serde(rename = "action-loc-key", skip_serializing_if = "Option::is_none")]
    pub action_loc_key: Option<&'a str>,
    #[serde(rename = "loc-key", skip_serializing_if = "Option::is_none")]
    pub loc_key: Option<&'a str>,
    #[serde(rename = "loc-args", skip_serializing_if = "Option::is_none")]
    pub loc_args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loc_image: Option<&'a str>,
}

impl<'a> AlertPayload<'a> {
    fn new(title: Option<&'a str>, body: Option<&'a str>) -> Self {
        AlertPayload {
            title: title,
            body: body,
            title_loc_key: None,
            title_loc_args: None,
            action_loc_key: None,
            loc_key: None,
            loc_args: None,
            loc_image: None,
        }
    }
}

/// The alert content.
/// This can either be a plain message string, or an AlertPayload with more
/// configuration.
#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Alert<'a> {
    Simple(&'a str),
    Payload(AlertPayload<'a>),
}

#[derive(Serialize, Default, Clone, Debug)]
pub struct Payload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<Alert<'a>>,
    /// Updates the numeric badge for the app. Set to 0 to remove.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<u32>,
    /// Sound to play. Use 'default' for the default sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<&'a str>,
    /// Set to true to mark the app as having content available.
    #[serde(rename = "content-available", skip_serializing_if = "Option::is_none")]
    pub content_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<&'a str>,
    #[serde(rename = "thread-id", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<&'a str>,
}

/// A full json request object for sending a notification to the API.
#[derive(Serialize, Clone, Debug)]
pub(crate) struct ApnsRequest<'a> {
    pub aps: &'a Payload<'a>,
    #[serde(flatten)]
    pub custom: Option<&'a HashMap<&'a str, &'a str>>,
}

/// A notification struct contains all relevant data for a notification request
/// sent to the APNS API.
/// This includes other options not contained in the payload.
/// These options are transferred with HTTP request headers.
#[derive(Serialize, Clone, Debug)]
pub struct Notification<'a> {
    /// The topic to use. Usually the app bundle id.
    pub topic: &'a str,
    pub device_token: &'a str,
    pub payload: Payload<'a>,

    /// Optional id identifying the message.
    pub id: Option<Uuid>,
    /// Optional expiration time as UNIX timestamp.
    pub expiration: Option<u64>,
    /// Priority for the notification.
    pub priority: Option<Priority>,
    pub collapse_id: Option<CollapseId<'a>>,
    pub apns_push_type: Option<ApnsPushType>,
    pub custom: Option<HashMap<&'a str, &'a str>>,
}

impl<'a> Notification<'a> {
    /// Create a new notification.
    pub fn new(topic: &'a str, device_token: &'a str, payload: Payload<'a>) -> Self {
        Notification {
            topic,
            device_token,
            payload,
            id: None,
            expiration: None,
            priority: None,
            collapse_id: None,
            apns_push_type: None,
            custom: None,
        }
    }
}

/// A builder for convenient construction of notifications.
pub struct NotificationBuilder<'a> {
    notification: Notification<'a>,
}

impl<'a> NotificationBuilder<'a> {
    pub fn new(topic: &'a str, device_id: &'a str) -> Self {
        NotificationBuilder {
            notification: Notification::new(topic, device_id, Payload::default()),
        }
    }

    pub fn push_type(&mut self, push_type: ApnsPushType) -> &mut Self {
        self.notification.apns_push_type = push_type.into();
        self
    }

    pub fn payload(&mut self, payload: Payload<'a>) -> &mut Self {
        self.notification.payload = payload;
        self
    }

    pub fn alert(&mut self, alert: &'a str) -> &mut Self {
        self.notification.payload.alert = Some(Alert::Simple(alert.into()));
        self
    }

    pub fn title(&mut self, title: &'a str) -> &mut Self {
        let payload = match self.notification.payload.alert.take() {
            None => AlertPayload::new(Some(title), None),
            Some(Alert::Simple(_)) => AlertPayload::new(Some(title), None),
            Some(Alert::Payload(mut payload)) => {
                payload.title = Some(title);
                payload
            }
        };
        self.notification.payload.alert = Some(Alert::Payload(payload));
        self
    }

    pub fn body(&mut self, body: &'a str) -> &mut Self {
        let payload = match self.notification.payload.alert.take() {
            None => AlertPayload::new(None, Some(body)),
            Some(Alert::Simple(title)) => AlertPayload::new(Some(title), Some(body)),
            Some(Alert::Payload(mut payload)) => {
                payload.body = Some(body);
                payload
            }
        };
        self.notification.payload.alert = Some(Alert::Payload(payload));
        self
    }

    pub fn badge(&mut self, number: u32) -> &mut Self {
        self.notification.payload.badge = Some(number);
        self
    }

    pub fn sound(&mut self, sound: &'a str) -> &mut Self {
        self.notification.payload.sound = Some(sound.into());
        self
    }

    pub fn content_available(&mut self) -> &mut Self {
        self.notification.payload.content_available = Some(true);
        self
    }

    pub fn category(&mut self, category: &'a str) -> &mut Self {
        self.notification.payload.category = Some(category);
        self
    }

    pub fn thread_id(&mut self, thread_id: &'a str) -> &mut Self {
        self.notification.payload.thread_id = Some(thread_id);
        self
    }

    pub fn id(&mut self, id: Uuid) -> &mut Self {
        self.notification.id = Some(id);
        self
    }

    pub fn expiration(&mut self, expiration: u64) -> &mut Self {
        self.notification.expiration = Some(expiration);
        self
    }

    pub fn priority(&mut self, priority: Priority) -> &mut Self {
        self.notification.priority = Some(priority);
        self
    }

    pub fn collapse_id(&mut self, id: CollapseId<'a>) -> &mut Self {
        self.notification.collapse_id = Some(id);
        self
    }

    pub fn custom(&mut self, custom: HashMap<&'a str, &'a str>) -> &mut Self {
        self.notification.custom = Some(custom);
        self
    }

    pub fn build(self) -> Notification<'a> {
        self.notification
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub reason: ApiErrorReason,
    pub timestamp: Option<i64>,
    pub token: String,

    pub apns_id: String,
    #[serde(skip)]
    pub(crate) status_code: StatusCode,
}

#[derive(Debug, Default)]
pub struct BatchResponse {
    pub success: i64,
    pub failure: i64,
    pub responses: Vec<Response>,
}