use reqwest::{header, Client};
use serde_json::json;
use std::env;

const GITHUB_URL: &str = "https://api.github.com/repos/GiganticMinecraft/SeichiAssist/issues";

pub struct GitHubClient {
    reqwest_client: Client,
    api_key: String,
}

impl GitHubClient {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        GitHubClient {
            reqwest_client: Client::new(),
            api_key: env::var("GITHUB_KEY").unwrap_or_default(),
        }
    }

    pub async fn create_issue(
        &self,
        title: &str,
        content: &str,
        labels: Vec<&str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let content = json!({
            "title": title,
            "content": content,
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
