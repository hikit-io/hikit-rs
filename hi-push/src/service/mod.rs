pub mod model;

#[cfg(feature = "mongo")]
pub mod mongo;

use anyhow::anyhow;
use std::collections::HashMap;

#[cfg(feature = "mongo")]
pub use mongo::*;

use crate::{apns, email, fcm, huawei, rtm, service::model::Body, xiaomi};

#[cfg(feature = "wecom")]
use crate::wecom;

pub enum Database {
    #[cfg(feature = "mongo")]
    Mongo(mongodb::Database),
}

pub enum Message {
    Transparent(model::PushTransparentParams),
    Notification(model::PushNotificationParams),
}

pub struct App {
    service: super::Service,
    #[cfg(feature = "mongo")]
    db: Database,
    channels: Vec<model::Channel>,
}

impl App {
    #[cfg(feature = "mysql")]
    pub async fn new(db: sea_orm::Database) -> App {
        let svcs = super::Service::new();
        Self {
            service: svcs,
            db,
            channels: Vec::new(),
        }
    }

    #[cfg(feature = "mongo")]
    pub async fn new(mongodb: mongodb::Database) -> App {
        let svcs = super::Service::new();
        Self {
            service: svcs,
            db: Database::Mongo(mongodb),
            channels: Vec::new(),
        }
    }

    pub async fn init(&mut self) -> anyhow::Result<()> {
        let chans = match &self.db {
            Database::Mongo(db) => fetch_all_channels(db).await?,
        };
        self.channels = chans;
        for chan in &self.channels {
            self.register_client(chan).await?;
        }
        Ok(())
    }

    /// Channel ids on running
    pub async fn running_ch_ids(&self) -> Vec<String> {
        let pushers = self.service.pushers.read().await;
        pushers.iter().map(|e| e.0.clone()).collect()
    }

    /// Add channel client to memory
    pub async fn register_client(&self, chan: &model::Channel) -> anyhow::Result<()> {
        match Self::new_client(chan).await {
            Ok(cli) => {
                tracing::info!("load channel:{}", chan.ch_id);
                self.service.register_client(&chan.ch_id, cli).await;
            }
            Err(e) => {
                tracing::error!("load channel error:`{}` {} ", chan.ch_id, e);
            }
        }
        Ok(())
    }

    /// Remove channel client from memory.
    pub async fn deregister_client(&self, ch_id: &str) {
        self.service.remove_client(ch_id).await;
    }

    /// Create a new channel client by config.
    pub async fn new_client(conf: &model::Channel) -> anyhow::Result<super::Client> {
        Ok(match conf._type {
            #[cfg(feature = "xiaomi")]
            model::ChannelType::Mi => super::Client::Mi(xiaomi::Client::new(&xiaomi::Config {
                client_id: conf
                    .client_id
                    .as_ref()
                    .ok_or(anyhow!("mi missing `client_id`"))?
                    .as_str(),
                client_secret: conf
                    .client_secret
                    .as_ref()
                    .ok_or(anyhow!("mi missing `client_secret`"))?
                    .as_str(),
                project_id: conf
                    .project_id
                    .as_ref()
                    .ok_or(anyhow!("mi missing `project_id`"))?
                    .as_str(),
            })?),
            #[cfg(feature = "huawei")]
            model::ChannelType::Huawei => super::Client::Huawei(
                huawei::Client::new(
                    conf.client_id
                        .as_ref()
                        .ok_or(anyhow!("mi missing `client_id`"))?
                        .as_str(),
                    conf.client_secret
                        .as_ref()
                        .ok_or(anyhow!("mi missing `client_id`"))?
                        .as_str(),
                )
                .await?,
            ),
            #[cfg(feature = "fcm")]
            model::ChannelType::Fcm => super::Client::Fcm(
                fcm::Client::new(fcm::Config {
                    key_type: Some(
                        conf.key_type
                            .clone()
                            .ok_or(anyhow!("Fcm missing `key_type`"))?,
                    ),
                    project_id: Some(
                        conf.project_id
                            .clone()
                            .ok_or(anyhow!("Fcm missing `project_id`"))?,
                    ),
                    private_key_id: Some(
                        conf.private_key_id
                            .clone()
                            .ok_or(anyhow!("Fcm missing `private_key_id`"))?,
                    ),
                    private_key: (conf
                        .private_key
                        .clone()
                        .ok_or(anyhow!("Fcm missing `private_key`"))?),
                    client_email: (conf
                        .client_email
                        .clone()
                        .ok_or(anyhow!("Fcm missing `client_email`"))?),
                    client_id: Some(
                        conf.client_id
                            .clone()
                            .ok_or(anyhow!("Fcm missing `client_id`"))?,
                    ),
                    auth_uri: Some(
                        conf.auth_uri
                            .clone()
                            .ok_or(anyhow!("Fcm missing `auth_uri`"))?,
                    ),
                    token_uri: conf
                        .token_uri
                        .clone()
                        .ok_or(anyhow!("Fcm missing `token_uri`"))?,
                    auth_provider_x509_cert_url: Some(
                        conf.auth_provider_x509_cert_url
                            .clone()
                            .ok_or(anyhow!("Fcm missing `auth_provider_x509_cert_url`"))?,
                    ),
                    client_x509_cert_url: Some(
                        conf.client_x509_cert_url
                            .clone()
                            .ok_or(anyhow!("Fcm missing `client_x509_cert_url`"))?,
                    ),
                })
                .await?,
            ),
            #[cfg(feature = "wecom")]
            model::ChannelType::Wecom => super::Client::Wecom(
                wecom::Client::new(
                    conf.client_id
                        .as_ref()
                        .ok_or(anyhow!("Wecom missing `client_id`"))?
                        .as_str(),
                    conf.client_secret
                        .as_ref()
                        .ok_or(anyhow!("Wecom missing `client_secret`"))?
                        .as_str(),
                    conf.agentid.ok_or(anyhow!("Wecom missing `agentid`"))?,
                )
                .await?,
            ),
            #[cfg(feature = "apns")]
            model::ChannelType::Apns => super::Client::Apns(apns::Client::new(
                conf.certs
                    .as_ref()
                    .ok_or(anyhow!("Apns missing `certs`"))?
                    .as_slice(),
                conf.client_secret
                    .as_ref()
                    .ok_or(anyhow!("Apns missing `client_secret`"))?
                    .as_str(),
            )?),
            #[cfg(feature = "email")]
            model::ChannelType::Email => super::Client::Email(
                email::Client::new(
                    conf.client_id
                        .as_ref()
                        .ok_or(anyhow!("Email missing `client_id`"))?
                        .as_str(),
                    conf.client_secret
                        .as_ref()
                        .ok_or(anyhow!("Email missing `client_secret`"))?
                        .as_str(),
                    conf.addr
                        .as_ref()
                        .ok_or(anyhow!("Email missing `addr`"))?
                        .as_str(),
                )
                .await,
            ),
            #[cfg(feature = "rtm")]
            model::ChannelType::Rtm => super::Client::Rtm(rtm::Client::new(
                conf.client_id
                    .as_ref()
                    .ok_or(anyhow!("Rtm missing `client_id`"))?
                    .as_str(),
                conf.client_secret
                    .as_ref()
                    .ok_or(anyhow!("Rtm missing `client_secret`"))?
                    .as_str(),
            )?),
            model::ChannelType::Unknown => Err(anyhow!("Unknown channel type"))?,
        })
    }

    /// Push message
    pub async fn push_message(
        &self,
        client_id: &str,
        msg: Message,
    ) -> anyhow::Result<model::PushResp> {
        let (groups, channels) = match &msg {
            Message::Transparent(msg) => (msg.groups.as_slice(), msg.channels.as_slice()),
            Message::Notification(msg) => (msg.groups.as_slice(), msg.channels.as_slice()),
        };

        let tokens = match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => {
                valid_client_id_and_ch_ids(db, client_id, channels).await?;

                fetch_tokens(db, channels, groups).await?
            }
        };

        let mut token_map: HashMap<&str, Vec<&str>> = HashMap::new();

        // collect token by channel
        for token in &tokens {
            if let Some(vec) = token_map.get_mut(token.ch_id.as_str()) {
                vec.push(&token.token);
            } else {
                let vec = vec![token.token.as_str()];
                token_map.insert(&token.ch_id, vec);
            }
        }

        let mut push_results = model::PushResp {
            success: 0,
            failure: 0,
            results: Default::default(),
        };

        let body = match &msg {
            Message::Transparent(msg) => match &msg.body {
                Body::Json(_) => super::Body::Transparent(""),
                Body::Text(text) => super::Body::Transparent(text),
            },
            Message::Notification(msg) => super::Body::Notify {
                title: msg.title.as_str(),
                body: msg.body.as_str(),
            },
        };

        let wecom = match &msg {
            Message::Transparent(msg) => msg.platform_extra.wecom.as_ref(),
            Message::Notification(msg) => msg.platform_extra.wecom.as_ref(),
        }
        .map_or(None, |wecom| {
            match wecom {
                model::WecomExtra::Markdown(ok) => crate::WecomExtra::Markdown(*ok),
                model::WecomExtra::Text { url, btntxt } => crate::WecomExtra::Text {
                    url: &url,
                    btntxt: &btntxt,
                },
            }
            .into()
        });

        let apns = match &msg {
            Message::Transparent(msg) => msg.platform_extra.apns.as_ref(),
            Message::Notification(msg) => msg.platform_extra.apns.as_ref(),
        }
        .map_or(None, |model::ApnsExtra { topic, push_type }| {
            crate::ApnsExtra {
                topic: topic,
                push_type: push_type,
            }
            .into()
        });

        let android = match &msg {
            Message::Transparent(msg) => msg.platform_extra.android.as_ref(),
            Message::Notification(msg) => msg.platform_extra.android.as_ref(),
        }
        .map_or(
            None,
            |model::AndroidExtra {
                 collapse_key,
                 priority,
                 ttl,
                 title,
                 body,
                 icon,
                 color,
                 sound,
                 tag,
                 click_action,
                 body_loc_key,
                 body_loc_args,
                 title_loc_key,
                 title_loc_args,
                 channel_id,
                 image,
                 ticker,
                 visibility,
                 package_name,
                 auto_clear,
                 foreground_show,
                 notify_id,
             }| {
                crate::AndroidExtra {
                    collapse_key: collapse_key.clone(),
                    priority: priority.clone(),
                    ttl: ttl.clone(),
                    title: title.as_deref(),
                    body: body.as_deref(),
                    icon: icon.as_deref(),
                    color: color.as_deref(),
                    sound: sound.as_deref(),
                    tag: tag.as_deref(),
                    click_action: click_action.as_deref(),
                    body_loc_key: body_loc_key.as_deref(),
                    body_loc_args: body_loc_args.as_deref(),
                    title_loc_key: title_loc_key.as_deref(),
                    title_loc_args: title_loc_args.as_deref(),
                    channel_id: channel_id.as_deref(),
                    image: image.as_deref(),
                    ticker: ticker.as_deref(),
                    visibility: visibility.clone(),
                    package_name: package_name.as_deref(),
                    auto_clear: auto_clear.clone(),
                    foreground_show: foreground_show.clone(),
                    notify_id: notify_id.clone(),
                }
                .into()
            },
        );

        // push message by channel
        for (chan, tokens) in token_map {
            let push_res = self
                .service
                .retry_batch_push(
                    &chan,
                    super::Message {
                        tokens: &tokens,
                        body: body,
                        android: android.as_ref(),
                        apns: apns.as_ref(),
                        wecom: wecom.as_ref(),
                    },
                )
                .await?;
            push_results.success += push_res.success;
            push_results.failure += push_res.failure;
            push_results.results.insert(chan.to_string(), push_res);
        }
        Ok(push_results)
    }

    /// Register token
    pub async fn register_token(
        &self,
        client_id: &str,
        group: &str,
        ch_id: &str,
        token: &str,
        _override: Option<bool>,
    ) -> anyhow::Result<()> {
        match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => {
                valid_client_id_and_ch_id(db, client_id, ch_id).await?;
                insert_token(db, client_id, ch_id, group, token, _override).await?
            }
        };
        Ok(())
    }

    /// Revoke token
    pub async fn revoke_token(
        &self,
        client_id: &str,
        group: &str,
        ch_id: &str,
        token: &str,
    ) -> anyhow::Result<()> {
        match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => {
                valid_client_id_and_ch_id(db, client_id, ch_id).await?;
                revoke_token(db, ch_id, group, token).await?
            }
        };
        Ok(())
    }

    /// Create a channel.
    pub async fn create_channel(
        &self,
        app_id: &str,
        params: model::PublicChannel,
    ) -> anyhow::Result<String> {
        let channel = match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => create_channel(db, app_id, params).await?,
        };
        let cli = Self::new_client(&channel).await?;
        let _ = self
            .service
            .register_client(channel.ch_id.as_str(), cli)
            .await;
        Ok(channel.ch_id)
    }

    /// Fetch channels from database belong client_id .
    pub async fn fetch_channels(&self, client_id: &str) -> anyhow::Result<Vec<model::Channel>> {
        let channels = match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => fetch_channels_by_client_id(db, client_id).await?,
        };
        Ok(channels)
    }

    /// Create application.
    pub async fn create_app(&self, name: &str) -> anyhow::Result<model::App> {
        let app = match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => create_app(db, name).await?,
        };
        Ok(app)
    }

    /// Fecth applications from database.
    pub async fn fetch_apps(&self) -> anyhow::Result<Vec<model::App>> {
        let apps = match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => fetch_apps(db).await?,
        };
        Ok(apps)
    }

    /// Delete application from database.
    pub async fn delete_app(&self, client_id: &str, client_secret: &str) -> anyhow::Result<()> {
        let _ = match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => delete_app(db, client_id, client_secret).await?,
        };
        Ok(())
    }

    /// Delete channel from database.
    pub async fn delete_channel(&self, client_id: &str, ch_id: &str) -> anyhow::Result<()> {
        let _ = match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => delete_channel(db, client_id, ch_id).await?,
        };
        self.deregister_client(ch_id).await;
        Ok(())
    }

    /// Validate client_id and client_secret.
    pub async fn validate_app(&self, client_id: &str, client_secret: &str) -> anyhow::Result<()> {
        let _ = match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => fetch_app(db, client_id, client_secret).await?,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
