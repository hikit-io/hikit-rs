pub mod model;

#[cfg(feature = "mongo")]
pub mod mongo;

use std::collections::HashMap;

#[cfg(feature = "mongo")]
pub use mongo::*;

use crate::{apns, fcm, huawei, xiaomi, PushResults};

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

pub struct App<'a> {
    svcs: super::Service<'a>,
    #[cfg(feature = "mongo")]
    db: Database,
    chans: Vec<model::Channel>,
}

impl<'a> App<'a> {
    pub async fn new(db: Database) -> App<'a> {
        let svcs = super::Service::new();
        Self {
            svcs,
            db,
            chans: Vec::new(),
        }
    }

    pub async fn init(&'a mut self) -> anyhow::Result<()> {
        let chans = match &self.db {
            Database::Mongo(db) => fetch_all_chans(db).await?,
        };
        self.chans = chans;
        for chan in &self.chans {
            let cli = Self::new_client(chan).await?;
            self.svcs.register_client(&chan.ch_id, cli).await;
        }
        Ok(())
    }

    pub async fn new_client<'b>(conf: &'b model::Channel) -> anyhow::Result<super::Client<'b>> {
        Ok(match conf._type {
            #[cfg(feature = "xiaomi")]
            model::ChannleType::Mi => super::Client::Mi(xiaomi::Client::new(&xiaomi::Config {
                client_id: &conf.client_id,
                client_secret: &conf.cliend_secret,
                project_id: &conf.project_id,
            })?),
            #[cfg(feature = "huawei")]
            model::ChannleType::Huawei => super::Client::Huawei(
                huawei::Client::new(&conf.client_id, &conf.cliend_secret).await?,
            ),
            #[cfg(feature = "fcm")]
            model::ChannleType::Fcm => super::Client::Fcm(
                fcm::Client::new(
                    fcm::Config {
                        key_type: Some(conf.key_type.clone()),
                        project_id: Some(conf.project_id.to_string()),
                        private_key_id: Some(conf.priviate_key_id.clone()),
                        private_key: (conf.priviate_key.clone()),
                        client_email: (conf.client_email.clone()),
                        client_id: Some(conf.client_id.clone()),
                        auth_uri: Some(conf.auth_uri.clone()),
                        token_uri: conf.token_uri.clone(),
                        auth_provider_x509_cert_url: Some(conf.auth_provider_x509_cert_url.clone()),
                        client_x509_cert_url: Some(conf.client_x509_cert_url.clone()),
                    }
                )
                    .await?,
            ),
            #[cfg(feature = "wecom")]
            model::ChannleType::Wecom => super::Client::Wecom(
                wecom::Client::new(&conf.client_id, &conf.cliend_secret, conf.agentid).await?,
            ),
            #[cfg(feature = "apns")]
            model::ChannleType::Apns => {
                super::Client::Apns(apns::Client::new(&conf.certs, &conf.cliend_secret)?)
            }
        })
    }

    pub async fn push_message(&self, msg: Message) -> anyhow::Result<()> {
        let (groups, chans) = match msg {
            Message::Transparent(msg) => (msg.groups, msg.chans),
            Message::Notification(msg) => (msg.groups, msg.chans),
        };

        let tokens = match &self.db {
            Database::Mongo(db) => fetch_tokens(db, &[""], &[""]).await?,
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

        let push_results = PushResults {
            success: 0,
            failure: 0,
            results: Vec::new(),
        };
        // push message by channel
        for (chan, tokens) in token_map {
            let push_res = self
                .svcs
                .retry_batch_push(
                    &chan,
                    super::Message {
                        tokens: &tokens,
                        body: todo!(),
                        android: todo!(),
                        apns: todo!(),
                        wecom: todo!(),
                    },
                )
                .await?;
            push_results.results.push(push_res);
        }
        Ok(())
    }

    pub async fn register_token(
        &self,
        group: &str,
        ch_id: &str,
        token: &str,
        overside: Option<bool>,
    ) -> anyhow::Result<()> {
        match &self.db {
            Database::Mongo(db) => insert_token(db, ch_id, group, token, overside).await?,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct Context<'a, 'b> {
        name: &'a str,
        vars: Vec<&'b str>,
    }

    #[test]
    fn test_it() {
        let mut ctx = Context {
            name: "",
            vars: Vec::default(),
        };
        {
            let b = String::from("dd");
            ctx.vars.push(&b);
        }
    }
}
