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

#[derive(Debug, Serialize_repr, Deserialize_repr, Default, Clone)]
#[repr(u8)]
pub enum Passtrough {
    Notice = 0,
    #[default]
    Transparent = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Default, Clone)]
#[repr(i8)]
pub enum NotifyType {
    All = -1,
    #[default]
    Quiet = 0,
    // 静默推送
    Sound = 1,
    Vibrate = 2,
    Lights = 4,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct Message<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<&'a str>,
    // 消息的内容。（注意：需要对payload字符串做urlencode处理）
    pub restricted_package_name: &'a str,
    // App的包名。备注：V2版本支持一个包名，V3版本支持多包名（中间用逗号分割）。
    pub pass_through: Passtrough,
    pub title: &'a str,
    // notify required <50
    pub description: &'a str,
    // notify required <128
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_type: Option<&'a str>,
    // notify_type的值可以是DEFAULT_ALL或者以下其他几种的OR组合：DEFAULT_ALL = -1;DEFAULT_SOUND = 1; // 使用默认提示音提示；DEFAULT_VIBRATE = 2; // 使用默认震动提示；DEFAULT_LIGHTS = 4; // 使用默认led灯光提示；
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<&'a str>,
    //可选项。如果用户离线，设置消息在服务器保存的时间，单位：ms。服务器默认最长保留两周。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_send: Option<&'a str>,
    //可选项。定时发送消息。用自1970年1月1日以来00:00:00.0 UTC时间表示（以毫秒为单位的时间）。注：仅支持七天内的定时消息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_id: Option<i32>,
    // 可选项。默认情况下，通知栏只显示一条推送消息。如果通知栏要显示多条推送消息，需要针对不同的消息设置不同的notify_id（相同notify_id的通知栏消息会覆盖之前的），且要求notify_id为取值在0~2147483647的整数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<String>,
    // 根据registration_id，发送消息到指定设备上。可以提供多个registration_id，发送给一组设备，不同的registration_id之间用“,”分割。参数仅适用于“/message/regid”HTTP API。（注意：需要对payload字符串做urlencode处理）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<&'a str>,
    // 根据alias，发送消息到指定的设备。可以提供多个alias，发送给一组设备，不同的alias之间用“,”分割。参数仅适用于“/message/alias”HTTP API。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_account: Option<&'a str>,
    // 根据user_account，发送消息给设置了该user_account的所有设备。可以提供多个user_account，user_account之间用“,”分割。参数仅适用于“/message/user_account”HTTP API。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<&'a str>,
    // 根据topic，发送消息给订阅了该topic的所有设备。参数仅适用于“/message/topic”HTTP API。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<&'a str>,
    // topic列表，使用;$;分割。注: topics参数需要和topic_op参数配合使用，另外topic的数量不能超过5。参数仅适用于“/message/multi_topic”HTTP API。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_op: Option<&'a str>,
    // topic之间的操作关系。支持以下三种：UNION并集INTERSECTION交集EXCEPT差集例如：topics的列表元素是[A, B, C, D]，则并集结果是A∪B∪C∪D，交集的结果是A ∩B ∩C ∩D，差集的结果是A-B-C-D。参数仅适用于“/message/multi_topic”HTTP API。
    #[serde(flatten)]
    pub extra: Option<Extra<'a>>,
}

#[derive(Debug, Serialize_repr, Default, Clone)]
#[repr(i16)]
pub enum CallbackType {
    #[default]
    ArriveClick = 3,

    Arrive = 1,
    // 送达
    Click = 2,
    InvalidDevice = 16,
    ClientForbidden = 32,
}

#[derive(Debug, Serialize_repr, Clone)]
#[repr(u8)]
pub enum NotifyEffect {
    Unkown = 0,
    Main = 1,
    Intent = 2,
    Web = 3,
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

#[derive(Debug, Serialize, Default, Clone)]
pub struct Extra<'a> {
    #[serde(rename = "extra.sound_uri", skip_serializing_if = "Option::is_none")]
    pub sound_uri: Option<&'a str>,
    // 可选项，自定义通知栏消息铃声。extra.sound_uri的值设置为铃声的URI。（请参见《服务端Java SDK文档》中的“自定义铃声”一节。）注：铃声文件放在Android app的raw目录下。
    #[serde(rename = "extra.ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<&'a str>,
    // 可选项，开启通知消息在状态栏滚动显示。
    #[serde(
        rename = "extra.notify_foreground",
        skip_serializing_if = "Option::is_none"
    )]
    pub notify_foreground: Option<Bool>,
    // 可选项，开启/关闭app在前台时的通知弹出。当extra.notify_foreground值为”1″时，app会弹出通知栏消息；当extra.notify_foreground值为”0″时，app不会弹出通知栏消息。注：默认情况下会弹出通知栏消息。（请参见《服务端Java SDK文档》中的“控制App前台通知弹出”一节。）
    #[serde(
        rename = "extra.notify_effect",
        skip_serializing_if = "Option::is_none"
    )]
    pub notify_effect: Option<NotifyEffect>,
    // 可选项，预定义通知栏消息的点击行为。通过设置extra.notify_effect的值以得到不同的预定义点击行为。“1″：通知栏点击后打开app的Launcher Activity。“2″：通知栏点击后打开app的任一Activity（开发者还需要传入extra.intent_uri）。“3″：通知栏点击后打开网页（开发者还需要传入extra.web_uri）。（请参见《服务端Java SDK文档》中的“预定义通知栏通知的点击行为”一节。）
    #[serde(rename = "extra.intent_uri", skip_serializing_if = "Option::is_none")]
    pub intent_uri: Option<&'a str>,
    // 可选项，打开当前app的任一组件。
    #[serde(rename = "extra.web_uri", skip_serializing_if = "Option::is_none")]
    pub web_uri: Option<&'a str>,
    // 可选项，打开某一个网页
    #[serde(rename = "extra.flow_control", skip_serializing_if = "Option::is_none")]
    pub flow_control: Option<&'a str>,
    // 可选项，控制是否需要进行平缓发送。默认不支持。value代表平滑推送的速度。注：服务端支持最低3000/s的qps。也就是说，如果将平滑推送设置为1000，那么真实的推送速度是3000/s。
    #[serde(
        rename = "extra.caljob_keylback",
        skip_serializing_if = "Option::is_none"
    )]
    pub job_key: Option<&'a str>,
    // 可选项，使用推送批次（JobKey）功能聚合消息。客户端会按照jobkey去重，即相同jobkey的消息只展示第一条，其他的消息会被忽略。合法的jobkey由数字（[0-9]），大小写字母（[a-zA-Z]），下划线（_）和中划线（-）组成，长度不大于20个字符，且不能以下划线(_)开头。
    #[serde(
        rename = "extra.only_send_once",
        skip_serializing_if = "Option::is_none"
    )]
    pub only_send_once: Option<&'a str>,
    //可选项，extra.only_send_once的值设置为1，表示该消息仅在设备在线时发送一次，不缓存离线消息进行多次下发。
    #[serde(rename = "extra.callback", skip_serializing_if = "Option::is_none")]
    pub callback: Option<&'a str>,
    #[serde(
        rename = "extra.callback.param",
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_param: Option<&'a str>,
    #[serde(
        rename = "extra.callback.type",
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_type: Option<CallbackType>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Data {
    pub id: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Response {
    pub result: String,
    pub description: String,
    pub data: Data,
    pub code: i32,
    pub info: String,
    pub reason: String,
}

impl<'a> Client {
    const PUSH_URL: &'static str = "https://api.xmpush.xiaomi.com/v4/message/regid";

    pub fn new(conf: &'a Config<'a>) -> Result<Self, super::Error> {
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
            .map_err(|e| super::InnerError::Http(e.to_string()))?;

        Ok(Client { cli })
    }
}

#[async_trait::async_trait]
impl<'b> super::Pusher<'b, Message<'b>, Response> for Client {
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
                21301 => Err(super::RetryError::Auth(format!("{:?}", resp)).into()),
                200001 => Err(super::InnerError::MessageLimit.into()),
                200002 => Err(super::RetryError::QPS.into()),
                _ => Err(super::InnerError::Response(format!("{:?}", resp)).into()),
            },
            Err(e) => Err(super::InnerError::Response(e.to_string()).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Pusher;

    #[tokio::test]
    async fn test_client() {
        use super::*;

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
