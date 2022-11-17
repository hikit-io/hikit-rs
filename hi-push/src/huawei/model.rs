use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr};

#[derive(Default, Debug, Serialize, Clone)]
pub struct Message<'a> {
    pub validate_only: bool,
    #[serde(borrow)]
    pub message: InnerMessage<'a>,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct InnerMessage<'a> {
    /*
       自定义消息负载，通知栏消息支持JSON格式字符串，透传消息支持普通字符串或者JSON格式字符串。样例："your data"，"{'param1':'value1','param2':'value2'}"。
       消息体中有message.data，没有message.notification和message.android.notification，消息类型为透传消息。
       如果用户发送的是网页应用的透传消息，那么接收消息中字段orignData为透传消息内容。
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<AndroidConfig<'a>>,
    #[serde(borrow)]
    pub token: Vec<&'a str>, // max: 1000 除开token，4096Bytes
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct Notification<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub image: Option<&'a str>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Urgency {
    High,
    Normal,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct AndroidConfig<'a> {
    /*
    0：对每个应用发送到该用户设备的离线消息只会缓存最新的一条。
    -1：对所有离线消息都缓存。默认值是-1
    1~100：离线消息缓存分组标识
    */
    pub collapse_key: Option<i64>,
    pub urgency: Option<Urgency>,
    pub category: Option<&'a str>,
    //消息缓存时间，单位是秒 例如： 1000s 。 默认值为“86400s”（1天），最大值为“1296000s”（15天）。
    pub ttl: Option<&'a str>,
    // 批量任务消息标识，消息回执时会返回给应用服务器，应用服务器可以识别bi_tag对消息的下发情况进行统计分析。
    pub bi_tag: Option<&'a str>,
    pub receipt_id: Option<&'a str>,
    /*
       快应用发送透传消息时，指定小程序的模式类型，小程序有两种模式开发态和生产态，取值如下：
       1：开发态
       2：生产态
       默认值是2。
    */
    pub fast_app_target: Option<i64>,
    pub data: Option<&'a str>,
    pub notification: Option<AndroidNotification<'a>>,
}

#[derive(Debug, Serialize_repr, Clone)]
#[repr(u8)]
pub enum Style {
    Default = 0,
    BigText = 1,
    Inbox = 2,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Importance {
    Low,
    Normal,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct AndroidNotification<'a> {
    pub image: Option<&'a str>,
    pub icon: Option<&'a str>,
    pub color: Option<&'a str>,
    pub sound: Option<&'a str>,
    pub default_sound: Option<bool>,
    pub tag: Option<&'a str>,
    pub importance: Option<Importance>,
    pub click_action: ClickAction<'a>,
    pub body_loc_key: Option<&'a str>,
    pub body_loc_args: Option<&'a [String]>,
    pub title_loc_key: Option<&'a str>,
    pub title_loc_args: Option<&'a [String]>,
    pub channel_id: Option<&'a str>,
    pub notify_summary: Option<&'a str>,
    pub style: Option<Style>,
    pub big_title: Option<&'a str>,
    pub big_body: Option<&'a str>,
    pub notify_id: Option<i64>,
    pub group: Option<&'a str>,
    pub badge: Option<Badge<'a>>,
    pub foreground_show: Option<bool>,
    pub ticker: Option<&'a str>,
    pub when: Option<&'a str>,
    pub local_only: Option<bool>,
    pub use_default_vibrate: Option<bool>,
    pub use_default_light: Option<bool>,
    pub visibility: Option<&'a str>,
    pub vibrate_config: Option<&'a [String]>,
    pub light_settings: Option<LightSettings<'a>>,
    pub auto_clear: Option<i8>,
}

#[derive(Debug, Serialize_repr, Default, Clone)]
#[repr(u8)]
pub enum ClickActionType {
    Intent = 1,
    Web = 2,
    #[default]
    Main = 3,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Code {
    #[serde(rename = "80000000")]
    Success,
    #[serde(rename = "80000001")]
    Common,
    #[serde(rename = "80100000")]
    PartFailedErr,
    #[serde(rename = "80100001")]
    ParameterError,
    #[serde(rename = "80100002")]
    TokenMustOne,
    #[serde(rename = "80100003")]
    MsgBodyError,
    #[serde(rename = "80100004")]
    TTLErr,
    #[serde(rename = "80200001")]
    AuthFailedErr,
    #[serde(rename = "80200003")]
    AuthTokenTimeoutErr,
    #[serde(rename = "80300007")]
    TokenInvalid,
    Other(String),
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct ClickAction<'a> {
    #[serde(rename = "type")]
    pub type_field: ClickActionType,
    pub intent: Option<&'a str>,
    pub url: Option<&'a str>,
    pub action: Option<&'a str>,
}

impl<'a> ClickAction<'a> {
    pub fn new_intent(action: &'a str) -> Self {
        Self {
            type_field: ClickActionType::Intent,
            action: Some(action),
            ..Default::default()
        }
    }
    pub fn new_web(url: &'a str) -> Self {
        Self {
            type_field: ClickActionType::Web,
            url: Some(url),
            ..Default::default()
        }
    }
    pub fn new_main() -> Self {
        Self {
            type_field: ClickActionType::Main,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct Badge<'a> {
    pub add_num: Option<i64>,
    pub class: &'a str,
    pub set_num: Option<i64>,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct LightSettings<'a> {
    pub color: Color,
    pub light_on_duration: &'a str,
    pub light_off_duration: &'a str,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct Color {
    pub alpha: Option<i64>,
    pub red: Option<i64>,
    pub green: Option<i64>,
    pub blue: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendResponse {
    pub msg: String,
    pub code: Code,
    pub request_id: String,
}

#[derive(Debug)]
pub struct Response {
    pub msg: String,
    pub code: Code,
    pub request_id: String,
    pub success: i64,
    pub failure: i64,
    pub illegal_tokens: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct InvalidMsg {
    pub success: i64,
    pub failure: i64,
    pub illegal_tokens: Vec<String>,
}

impl SendResponse {
    pub fn get_invalid_tokens(&self) -> Option<InvalidMsg> {
        serde_json::from_str::<InvalidMsg>(self.msg.as_str()).ok()
    }

    pub fn is_part_failed_err(&self) -> bool {
        self.code == Code::PartFailedErr
    }

    pub fn take_invalid_tokens(&self) -> (Option<InvalidMsg>, bool) {
        (
            serde_json::from_str::<InvalidMsg>(self.msg.as_str()).ok(),
            self.is_part_failed_err(),
        )
    }
}