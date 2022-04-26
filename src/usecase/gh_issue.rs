use crate::domain::{gh_issue::Issue, repository::GitHubIssueRepository};
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct GitHubIssueUseCase<R: GitHubIssueRepository> {
    repository: Arc<R>,
}

impl<R: GitHubIssueRepository> GitHubIssueUseCase<R> {
    pub async fn add(&self, issue: Issue) -> anyhow::Result<String> {
        self.repository.add(issue).await
    }
}
