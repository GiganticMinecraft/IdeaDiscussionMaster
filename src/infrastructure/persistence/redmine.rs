use crate::util::Env;
use reqwest::Client;
use std::sync::Arc;

pub struct Redmine {
    pub client: Arc<Client>,
    pub token: String,
}

impl Redmine {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Client::new()),
            token: format!("key={}", Env::new().redmine_api_key),
        }
    }
}

impl Default for Redmine {
    fn default() -> Self {
        Self::new()
    }
}
