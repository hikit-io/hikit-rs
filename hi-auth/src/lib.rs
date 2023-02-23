use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
    hash::Hash,
};

use async_trait::async_trait;

pub mod github;

pub mod wechat;

pub mod wecom;

pub mod qq;

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Userinfo {
    pub unique_id: String,
    pub name: String,
    pub account: String,
    pub email: Option<String>,
    pub organization: Option<Vec<Organization>>,
}

pub struct Organization {
    pub unique_id: String,
    pub account: String,
    pub name: String,
}

pub enum Client {
    Github(github::Client),
}

pub struct Service<T>
    where
        T: Eq + Hash,
{
    clients: HashMap<T, Client>,
}

impl<T> Service<T>
    where
        T: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn register(&mut self, index: T, cli: Client) {
        self.clients.insert(index, cli);
    }

    pub fn deregister(&mut self, index: T) {
        self.clients.remove(&index);
    }

    pub async fn userinfo(&self, index: &T, code: &str) -> Result<Userinfo> {
        let cli = self.clients.get(index).unwrap();
        match cli {
            Client::Github(gh) => gh.userinfo(code).await,
        }
    }
}

#[async_trait]
pub trait Profile {
    async fn userinfo(&self, code: &str) -> Result<Userinfo>;
}

#[cfg(test)]
mod tests {
    #[derive(Hash, Eq)]
    enum Type {
        Github = 0,
    }

    #[test]
    fn test_clients() {
        let mut mgr = super::Service::new();

        let gh = super::github::Client::new("", "");

        mgr.register(Type::Github, super::Client::Github(gh));

        let userinfo = mgr.userinfo(Type::Github, "");
    }
}
