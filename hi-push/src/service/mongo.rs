use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options;

use crate::utils::now_ts;

use super::model::*;

/// fetch_apps
pub async fn fetch_app(
    mongo: &mongodb::Database,
    client_id: &str,
    client_secret: &str,
) -> anyhow::Result<App> {
    mongo
        .collection("app")
        .find_one(
            doc! {
                "clientId":client_id,
                "clientSecret":client_secret,
            },
            None,
        )
        .await?
        .ok_or(anyhow::anyhow!("No App"))
}

/// fetch_channels
/// Fetch channel's config to register client by client_id.
pub async fn fetch_channels(
    mongo: &mongodb::Database,
    client_id: &str,
) -> anyhow::Result<Vec<Channel>> {
    Ok(mongo
        .collection("channel")
        .find(
            doc! {
                "appId":client_id,
            },
            None,
        )
        .await?
        .try_collect::<Vec<_>>()
        .await?)
}

/// fetch_all_channels
/// Fetch all channel config.
pub async fn fetch_all_channels(mongo: &mongodb::Database) -> anyhow::Result<Vec<Channel>> {
    Ok(mongo
        .collection("channel")
        .find(doc! {}, None)
        .await?
        .try_collect::<Vec<_>>()
        .await?)
}

pub async fn fetch_channels_by_client_id(
    mongo: &mongodb::Database,
    client_id: &str,
) -> anyhow::Result<Vec<Channel>> {
    Ok(mongo
        .collection("channel")
        .find(
            doc! {
                "appId":client_id,
            },
            None,
        )
        .await?
        .try_collect::<Vec<_>>()
        .await?)
}

/// fetch_tokens
pub async fn fetch_tokens(
    mongo: &mongodb::Database,
    ch_ids: &[String],
    groups: &[String],
) -> anyhow::Result<Vec<Token>> {
    Ok(mongo
        .collection("token")
        .find(
            doc! {
                "chId":doc! {
                    "$in":ch_ids,
                },
                "group":doc! {
                    "$in":groups,
                },
                "valid":true,
            },
            None,
        )
        .await?
        .try_collect::<Vec<_>>()
        .await?)
}

pub async fn valid_client_id_and_ch_id(
    mongo: &mongodb::Database,
    client_id: &str,
    ch_id: &str,
) -> anyhow::Result<()> {
    let _ = mongo
        .collection::<Channel>("channel")
        .find_one(
            doc! {
                "appId":client_id,
                "chId":ch_id,
            },
            None,
        )
        .await?
        .ok_or(anyhow::anyhow!("not exist on app: `{}`", ch_id))?;
    Ok(())
}

pub async fn valid_client_id_and_ch_ids(
    mongo: &mongodb::Database,
    client_id: &str,
    ch_ids: &[String],
) -> anyhow::Result<()> {
    let res = mongo
        .collection::<Channel>("channel")
        .find(
            doc! {
                "appId":client_id,
                "chId":doc!{
                    "$in":ch_ids
                },
            },
            None,
        )
        .await?;
    let channels = res.try_collect::<Vec<Channel>>().await?;
    let mut err = Vec::new();

    'E: for ch_id in ch_ids.into_iter() {
        for channel in channels.iter() {
            if channel.ch_id.eq(ch_id) {
                continue 'E;
            }
        }
        err.push(ch_id);
    }
    if !err.is_empty() {
        return Err(anyhow::anyhow!("ch_id not exist on your app:`{:?}`", err));
    }

    Ok(())
}

pub async fn insert_token(
    mongo: &mongodb::Database,
    client_id: &str,
    ch_id: &str,
    group: &str,
    token: &str,
    _override: Option<bool>,
) -> anyhow::Result<()> {
    if let Some(_override) = _override {
        if _override {
            let _ = mongo
                .collection::<Token>("token")
                .update_many(
                    doc! {
                        "appId": client_id,
                        "chId": ch_id,
                        "token": token,
                        "group": doc! {
                            "$ne": group,
                        }
                    },
                    doc! {
                        "$set":doc!{
                            "updateTs": chrono::Utc::now().timestamp(),
                            "valid": false,
                        }
                    },
                    None,
                )
                .await?;
        }
    }
    // if exist no action, or insert
    let _ = mongo
        .collection::<Token>("token")
        .update_one(
            doc! {
                "chId": ch_id,
                "group": group,
                "token": token,
                "appId": client_id,
                "valid": true,
            },
            doc! {
                "$setOnInsert": doc!{
                    "updateTs":chrono::Utc::now().timestamp(),
                    "createTs":chrono::Utc::now().timestamp(),
                }
            },
            options::UpdateOptions::builder().upsert(true).build(),
        )
        .await?;
    Ok(())
}

pub async fn revoke_token(
    mongo: &mongodb::Database,
    ch_id: &str,
    group: &str,
    token: &str,
) -> anyhow::Result<()> {
    let _ = mongo
        .collection::<Token>("token")
        .update_one(
            doc! {
                "chId":ch_id,
                "group":group,
                "token":token,
                "valid":true,
            },
            doc! {
                "$set":doc!{
                    "valid":false,
                    "updateTs":chrono::Utc::now().timestamp(),
                }
            },
            None,
        )
        .await?;
    Ok(())
}

pub async fn create_channel(
    db: &mongodb::Database,
    app_id: &str,
    pc: PublicChannel,
) -> anyhow::Result<Channel> {
    let channel = match pc {
        #[cfg(feature = "wecom")]
        PublicChannel::Wecom {
            client_id,
            client_secret,
            agentid,
        } => Channel {
            app_id: app_id.to_string(),
            _type: ChannelType::Wecom,
            client_id: Some(client_id),
            client_secret: Some(client_secret),
            agentid: Some(agentid),
            ..Default::default()
        },
        #[cfg(feature = "fcm")]
        PublicChannel::Fcm {
            key_type,
            private_key_id,
            private_key,
            client_email,
            auth_uri,
            token_uri,
            auth_provider_x509_cert_url,
            client_x509_cert_url,
        } => Channel {
            app_id: app_id.to_string(),
            _type: ChannelType::Fcm,

            key_type: Some(key_type),
            private_key: Some(private_key),
            private_key_id: Some(private_key_id),
            client_email: Some(client_email),
            auth_uri: Some(auth_uri),
            token_uri: Some(token_uri),
            auth_provider_x509_cert_url: Some(auth_provider_x509_cert_url),
            client_x509_cert_url: Some(client_x509_cert_url),
            ..Default::default()
        },
        #[cfg(feature = "email")]
        PublicChannel::Email {
            client_id,
            client_secret,
            addr,
        } => Channel {
            app_id: app_id.to_string(),
            _type: ChannelType::Email,

            client_id: Some(client_id),
            client_secret: Some(client_secret),
            addr: Some(addr),
            ..Default::default()
        },
        #[cfg(feature = "xiaomi")]
        PublicChannel::Xiaomi {
            client_id,
            client_secret,
        } => Channel {
            app_id: app_id.to_string(),
            _type: ChannelType::Mi,

            client_id: Some(client_id),
            client_secret: Some(client_secret),
            ..Default::default()
        },
        #[cfg(feature = "apns")]
        PublicChannel::Apns {
            client_id,
            client_secret,
        } => {
            let certs = base64::decode(client_secret)?;
            Channel {
                app_id: app_id.to_string(),
                _type: ChannelType::Apns,

                client_id: Some(client_id),
                certs: Some(certs),
                ..Default::default()
            }
        }
        #[cfg(feature = "huawei")]
        PublicChannel::Huawei {
            client_id,
            client_secret,
        } => Channel {
            app_id: app_id.to_string(),
            _type: ChannelType::Huawei,

            client_id: Some(client_id),
            client_secret: Some(client_secret),
            ..Default::default()
        },
        #[cfg(feature = "rtm")]
        PublicChannel::AgoraRtm {
            client_id,
            client_secret,
        } => Channel {
            app_id: app_id.to_string(),
            _type: ChannelType::Rtm,

            client_id: Some(client_id),
            client_secret: Some(client_secret),
            ..Default::default()
        },
    };
    let _ = db
        .collection::<Channel>("channel")
        .insert_one(&channel, None)
        .await?;
    Ok(channel)
}

pub async fn create_app(db: &mongodb::Database, name: &str) -> anyhow::Result<App> {
    let client_id = uuid::Uuid::new_v4().simple().to_string();
    let client_secret = uuid::Uuid::new_v4().simple().to_string();

    let app = App {
        id: Default::default(),
        name: name.to_string(),
        client_id,
        client_secret,
        create_ts: now_ts(),
        update_ts: now_ts(),
    };
    db.collection::<App>("app").insert_one(&app, None).await?;
    Ok(app)
}

pub async fn fetch_apps(db: &mongodb::Database) -> anyhow::Result<Vec<App>> {
    let apps = db.collection::<App>("app").find(doc! {}, None).await?;
    Ok(apps.try_collect::<Vec<App>>().await?)
}

pub async fn delete_app(
    db: &mongodb::Database,
    client_id: &str,
    client_secret: &str,
) -> anyhow::Result<()> {
    let _ = db
        .collection::<App>("app")
        .delete_one(
            doc! {
                "clientId":client_id,
                "clientSecret":client_secret,
            },
            None,
        )
        .await?;
    let _ = delete_channel_by_client_id(db, client_id).await?;
    Ok(())
}

async fn delete_channel_by_client_id(
    db: &mongodb::Database,
    client_id: &str,
) -> anyhow::Result<()> {
    let _ = db
        .collection::<Channel>("channel")
        .delete_many(
            doc! {
                "appId":client_id,
            },
            None,
        )
        .await?;
    Ok(())
}

pub async fn delete_channel(
    db: &mongodb::Database,
    client_id: &str,
    ch_id: &str,
) -> anyhow::Result<()> {
    let _ = db
        .collection::<Channel>("app")
        .delete_one(
            doc! {
                "clientId":client_id,
                "chId":ch_id,
            },
            None,
        )
        .await?;
    Ok(())
}
