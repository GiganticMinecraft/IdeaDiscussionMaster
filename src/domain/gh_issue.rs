use derive_new::new;
use serde::Deserialize;

#[derive(new)]
pub struct Issue {
    pub title: String,
    pub content: String,
    pub labels: Vec<String>,
}

// TODO: ここではない
/// GitHubにIssueを作成したときのResponseをデシリアライズするための構造体
#[derive(Deserialize)]
pub struct CreateIssueResponse {
    /// 作成したIssueのURL
    pub html_url: String,
}
