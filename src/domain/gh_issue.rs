use serde::Deserialize;

pub struct Issue {
    pub title: String,
    pub content: String,
    pub labels: Vec<String>,
}

impl Issue {
    pub fn new(title: String, content: String, labels: Vec<String>) -> Self {
        Self {
            title,
            content,
            labels,
        }
    }
}

// TODO: ここではない
/// GitHubにIssueを作成したときのResponseをデシリアライズするための構造体
#[derive(Deserialize)]
pub struct CreateIssueResponse {
    /// 作成したIssueのURL
    pub html_url: String,
}
