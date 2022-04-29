use crate::util::Env;
use create_github_app_token::publish_token;
use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct GitHub {
    pub client: Arc<Client>,
    pub token: String,
}

impl GitHub {
    pub async fn new() -> Self {
        Self {
            client: Arc::new(Client::new()),
            token: format!("token {}", Self::create_token().await.unwrap()),
        }
    }

    async fn create_token() -> anyhow::Result<String> {
        let env = Env::new();
        let path = env.github_secret_key;
        let app_id = env.github_app_id;

        Ok(publish_token(app_id, path, "GiganticMinecraft")
            .await?
            .token)
    }
}
