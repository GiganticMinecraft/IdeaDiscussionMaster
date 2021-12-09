use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateIssueResponse {
    pub html_url: String,
}
