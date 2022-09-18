use crate_shared::Env;

use create_github_app_token::publish_token;
use derive_new::new;
use std::sync::Arc;
use surf::Client;

pub struct GitHubClient {
    pub client: Arc<Client>,
    pub token: String,
}

impl GitHubClient {
    pub fn new() -> Self {
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
