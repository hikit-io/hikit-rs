use futures::TryStreamExt;
use mongodb::bson::doc;

use super::model::*;

pub async fn fetch_apps(
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

pub async fn fetch_chans(
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

pub async fn fetch_all_chans(mongo: &mongodb::Database) -> anyhow::Result<Vec<Channel>> {
    Ok(mongo
        .collection("channel")
        .find(doc! {}, None)
        .await?
        .try_collect::<Vec<_>>()
        .await?)
}

pub async fn fetch_tokens(
    mongo: &mongodb::Database,
    ch_ids: &[&str],
    groups: &[&str],
) -> anyhow::Result<Vec<Token>> {
    Ok(mongo
        .collection("token")
        .find(
            doc! {
                "ch_id":doc! {
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

pub async fn insert_token(
    mongo: &mongodb::Database,
    ch_id: &str,
    group: &str,
    token: &str,
    overside: Option<bool>,
) -> anyhow::Result<()> {
    let _ = mongo
        .collection::<Token>("token")
        .insert_one(
            &Token {
                ch_id: ch_id.to_string(),
                token: token.to_string(),
                group: group.to_string(),
                ..Default::default()
            },
            None,
        )
        .await?;

    if let Some(overside) = overside {
        if overside {
            let _ = mongo
                .collection::<Token>("token")
                .update_many(
                    doc! {
                        "chId": ch_id,
                        "token": token,
                    },
                    doc! {
                        "$set":doc!{
                            "updateTs":Token::default().update_ts,
                            "valid": false,
                        }
                    },
                    None,
                )
                .await?;
        }
    }
    Ok(())
}

pub async fn insert_chan(db: &mongodb::Database) -> anyhow::Result<()> {
    let ires = db
        .collection("channel")
        .insert_one(
            Channel {
                id: todo!(),
                ch_id: todo!(),
                app_id: todo!(),
                _type: todo!(),
                client_id: todo!(),
                client_secret: todo!(),
                project_id: todo!(),
                certs: todo!(),
                agentid: todo!(),
                key_type: todo!(),
                private_key_id: todo!(),
                private_key: todo!(),
                client_email: todo!(),
                auth_uri: todo!(),
                token_uri: todo!(),
                auth_provider_x509_cert_url: todo!(),
                client_x509_cert_url: todo!(),
            },
            None,
        )
        .await;

    Ok(())
}
