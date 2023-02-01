use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Admin {
    #[cfg(feature = "mongo")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub password: String,
    pub update_ts: i64,
    pub create_ts: i64,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Default, Clone)]
#[repr(i32)]
pub enum ChannelType {
    #[default]
    Unknown = 0,
    #[cfg(feature = "xiaomi")]
    Mi = 1,
    #[cfg(feature = "huawei")]
    Huawei = 2,
    #[cfg(feature = "fcm")]
    Fcm = 3,
    #[cfg(feature = "wecom")]
    Wecom = 4,
    #[cfg(feature = "apns")]
    Apns = 5,
    #[cfg(feature = "email")]
    Email = 6,
}

/*
    App
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct App {
    #[cfg(feature = "mongo")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub client_id: String,
    pub client_secret: String,

    pub create_ts: i64,
    pub update_ts: i64,
}

/*
    Channel
    key_type: todo!(),
    project_id: Some(conf.project_id.to_string()),
    private_key_id: todo!(),
    private_key: todo!(),
    client_email: todo!(),
    client_id: todo!(),
    auth_uri: todo!(),
    token_uri: todo!(),
    auth_provider_x509_cert_url: todo!(),
    client_x509_cert_url: todo!(),
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    #[cfg(feature = "mongo")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub ch_id: String,
    pub app_id: String,
    #[serde(rename = "type")]
    pub _type: ChannelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    //
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    // apple
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certs: Option<Vec<u8>>,
    // wecom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agentid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_provider_x509_cert_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_x509_cert_url: Option<String>,
    // email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,

    pub create_ts: i64,
    pub update_ts: i64,
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            #[cfg(feature = "mongo")]
            id: Default::default(),
            ch_id: uuid::Uuid::new_v4().to_string(),
            app_id: "".to_string(),
            _type: Default::default(),
            client_id: None,
            client_secret: None,
            project_id: None,
            certs: None,
            agentid: None,
            key_type: None,
            private_key_id: None,
            private_key: None,
            client_email: None,
            auth_uri: None,
            token_uri: None,
            auth_provider_x509_cert_url: None,
            client_x509_cert_url: None,
            addr: None,
            create_ts: chrono::Utc::now().timestamp(),
            update_ts: chrono::Utc::now().timestamp(),
        }
    }
}

/*
    Token
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[cfg(feature = "mongo")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub app_id: String,
    pub ch_id: String,
    pub group: String,
    pub token: String,
    pub valid: bool,
    pub create_ts: i64,
    pub update_ts: i64,
}

#[cfg(not(feature = "client"))]
impl Default for Token {
    fn default() -> Self {
        Self {
            #[cfg(feature = "mongo")]
            id: Default::default(),
            ch_id: Default::default(),
            token: Default::default(),
            valid: true,
            create_ts: chrono::Utc::now().timestamp(),
            update_ts: chrono::Utc::now().timestamp(),
            group: Default::default(),
            app_id: Default::default(),
        }
    }
}
