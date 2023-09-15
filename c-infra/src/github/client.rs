use crate_shared::Env;

use create_github_app_token::publish_token;
use std::sync::Arc;
use surf::Client;

pub struct GitHubClient {
    pub client: Arc<Client>,
    pub token: String,
}

impl GitHubClient {
    pub async fn new() -> Self {
        let token = Self::create_token()
            .await
            .unwrap_or_else(|e| panic!("Error while opening GitHub app secret key: {}", e));

        Self {
            client: Arc::new(Client::new()),
            token: format!("token {}", token),
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
