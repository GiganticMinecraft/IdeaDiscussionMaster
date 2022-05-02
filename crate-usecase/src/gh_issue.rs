use crate_domain::{github::Issue, repository::GitHubIssueRepository};
use crate_infrastructure::module::RepositoryModuleExt;

use derive_new::new;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct GitHubIssueUseCase<R: RepositoryModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoryModuleExt> GitHubIssueUseCase<R> {
    pub async fn add(&self, issue: Issue) -> anyhow::Result<String> {
        self.repositories.github_issue_repository().add(issue).await
    }
}
