use serde::Deserialize;

/// GitHubにIssueを作成したときのResponseをデシリアライズするための構造体
#[derive(Deserialize)]
pub struct CreateIssueResponse {
    /// 作成したIssueのURL
    pub html_url: String,
}
