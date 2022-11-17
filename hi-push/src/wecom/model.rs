use std::sync::Arc;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub errcode: i32,
    pub errmsg: String,
    pub invaliduser: Option<String>,
    pub invalidparty: Option<String>,
    pub invalidtag: Option<String>,
    pub msgid: String,
    pub response_code: Option<String>,
}


#[derive(Debug, Deserialize, Clone)]
pub struct TokenResponse {
    pub errcode: i32,
    pub errmsg: String,
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize_repr, Clone)]
#[repr(u8)]
pub enum Bool {
    False = 0,
    True = 1,
}

impl From<Bool> for bool {
    fn from(b: Bool) -> Self {
        match b {
            Bool::False => false,
            Bool::True => true,
        }
    }
}

impl From<bool> for Bool {
    fn from(b: bool) -> Self {
        match b {
            true => Self::True,
            false => Self::False,
        }
    }
}

#[derive(Debug, Serialize, Builder)]
pub struct Message<'a> {
    #[serde(flatten)]
    pub to: To<'a>,
    pub msgtype: MsgType,
    #[builder(default)]
    pub agentid: i64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<Bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_duplicate_check: Option<Bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_check_interval: Option<Bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_id_trans: Option<Bool>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    #[builder(setter(custom), default)]
    pub inner: Option<InnerMesssage<'a>>,
}

impl<'a> MessageBuilder<'a> {
    pub fn inner(&mut self, value: InnerMesssage<'a>) -> &mut Self {
        match value {
            InnerMesssage::Text { .. } => self.msgtype = Some(MsgType::Text),
            InnerMesssage::Markdown { .. } => self.msgtype = Some(MsgType::Markdown),
            InnerMesssage::Textcard { .. } => self.msgtype = Some(MsgType::Textcard),
        }
        self.inner = Some(Some(value));
        self
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum To<'a> {
    Touser(String),
    Toparty(&'a str),
    Totag(&'a str),
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum MsgType {
    #[default]
    Text,
    Markdown,
    Textcard,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum InnerMesssage<'a> {
    Text {
        content: &'a str,
    },
    Markdown {
        content: &'a str,
    },
    Textcard {
        title: &'a str,
        description: &'a str,
        url: &'a str,
        btntxt: Option<&'a str>,
    },
}