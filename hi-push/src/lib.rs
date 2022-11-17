#[cfg(all(feature = "mysql", feature = "mongo"))]
compile_error!("mysql and mongo not enable both.");

#[cfg(all(feature = "http-api", not(any(feature = "wecom", feature = "fcm", feature = "email", feature = "apns", feature = "huawei", feature = "xiaomi"))))]
compile_error!("enable one of mysql and mongo");

#[cfg(all(feature = "http-api", not(any(feature = "mysql", feature = "mongo"))))]
compile_error!("enable one of mysql and mongo");

#[cfg(any(feature = "wecom", feature = "fcm", feature = "email", feature = "apns", feature = "huawei", feature = "xiaomi"))]
mod utils;

#[cfg(any(feature = "apns", feature = "apns-model"))]
pub mod apns;
#[cfg(any(feature = "email", feature = "email-model"))]
pub mod email;
#[cfg(any(feature = "fcm", feature = "fcm-model"))]
pub mod fcm;
#[cfg(any(feature = "huawei", feature = "huawei-model"))]
pub mod huawei;
#[cfg(any(feature = "wecom", feature = "wecom-model"))]
pub mod wecom;
#[cfg(any(feature = "xiaomi", feature = "xiaomi-model"))]
pub mod xiaomi;
#[cfg(all(feature = "grpc-api", any(feature = "wecom", feature = "xiaomi", feature = "fcm", feature = "huawei", feature = "apns", feature = "email")))]
pub mod grpc;
#[cfg(all(feature = "http-api", any(feature = "wecom", feature = "xiaomi", feature = "fcm", feature = "huawei", feature = "apns", feature = "email")))]
pub mod http;

#[cfg(feature = "client")]
pub mod client;

#[cfg(all(any(feature = "http-api", feature = "grpc-api", feature = "client")))]
pub mod service;

#[cfg(any(
feature = "wecom",
feature = "xiaomi",
feature = "fcm",
feature = "huawei",
feature = "apns",
feature = "email",
feature = "client",
))]
mod libs;

#[cfg(any(
feature = "wecom",
feature = "xiaomi",
feature = "fcm",
feature = "huawei",
feature = "apns",
feature = "email",
feature = "client",
))]
pub use libs::{*};
