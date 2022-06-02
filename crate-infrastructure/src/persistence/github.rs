use crate_shared::Env;

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

        Ok(
            publish_token(env.gh_app_id, "./key.pem", "GiganticMinecraft")
                .await?
                .token,
        )
    }
}
