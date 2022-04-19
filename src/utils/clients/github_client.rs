use crate::utils;
use create_github_app_token::{errors::Error as CError, publish_token};
use reqwest::{header, Client};
use serde_json::json;

const GITHUB_URL: &str = "https://api.github.com/repos/GiganticMinecraft/SeichiAssist/issues";

pub struct GitHubClient {
    reqwest_client: Client,
    api_key: String,
}

impl GitHubClient {
    #[allow(clippy::new_without_default)]
    pub async fn new() -> Self {
        GitHubClient {
            reqwest_client: Client::new(),
            api_key: Self::create_token().await.unwrap_or_default(),
        }
    }

    async fn create_token() -> Result<String, CError> {
        let env = utils::Env::new();
        let path = env.github_secret_key;
        let app_id = env.github_app_id;

        Ok(publish_token(app_id, path, "GiganticMinecraft")
            .await?
            .token)
    }

    pub async fn create_issue(
        &self,
        title: &str,
        content: &str,
        labels: Vec<&str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let content = json!({
            "title": title,
            "body": content,
            "labels": labels
        });

        self.reqwest_client
            .post(GITHUB_URL)
            .header(header::USER_AGENT, "curl/7.38.0")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/vnd.github.v3+json")
            .header(header::AUTHORIZATION, format!("token {}", self.api_key))
            .json(&content)
            .send()
            .await
    }
}