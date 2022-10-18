pub mod model;

#[cfg(feature = "mongo")]
pub mod mongo;

use anyhow::anyhow;
use std::collections::HashMap;

#[cfg(feature = "mongo")]
pub use mongo::*;

use crate::service::model::Body;
use crate::{apns, email, fcm, huawei, rtm, xiaomi, PushResults};

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

    pub async fn running_ch_ids(&self) -> Vec<String> {
        let pushers = self.service.pushers.read().await;
        pushers.iter().map(|e| e.0.clone()).collect()
    }

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

    pub async fn push_message(&self, client_id: &str, msg: Message) -> anyhow::Result<()> {
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

        let mut push_results = PushResults {
            success: 0,
            failure: 0,
            results: Vec::new(),
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

        // push message by channel
        for (chan, tokens) in token_map {
            let push_res = self
                .service
                .retry_batch_push(
                    &chan,
                    super::Message {
                        tokens: &tokens,
                        body: body,
                        android: None,
                        apns: None,
                        wecom: None,
                    },
                )
                .await?;
            println!("{push_res:?}");
            // push_results.results.push(push_res);
        }
        Ok(())
    }

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

    pub async fn create_channel(
        &self,
        app_id: &str,
        params: model::PublicChannel,
    ) -> anyhow::Result<()> {
        let channel = match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => create_channel(db, app_id, params).await?,
        };
        let cli = Self::new_client(&channel).await?;
        let _ = self
            .service
            .register_client(channel.ch_id.as_str(), cli)
            .await;
        Ok(())
    }

    pub async fn create_app(&self, name: &str) -> anyhow::Result<model::App> {
        let app = match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => create_app(db, name).await?,
        };
        Ok(app)
    }

    pub async fn valid_app(&self, client_id: &str, client_secret: &str) -> anyhow::Result<()> {
        let app = match &self.db {
            #[cfg(feature = "mongo")]
            Database::Mongo(db) => fetch_app(db, client_id, client_secret).await?,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // #[derive(Debug)]
    // struct Context<'a, 'b> {
    //     name: &'a str,
    //     vars: Vec<&'b str>,
    // }
    //
    // #[test]
    // fn test_it() {
    //     let mut ctx = Context {
    //         name: "",
    //         vars: Vec::default(),
    //     };
    //     {
    //         let b = String::from("dd");
    //         ctx.vars.push(&b);
    //     }
    // }
}
