use crate_domain::{
    github::Issue,
    repository::{GitHubIssueRepository, RepositoryModuleExt},
};

use anyhow::Context as _;
use derive_new::new;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct GitHubIssueUseCase<R: RepositoryModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoryModuleExt> GitHubIssueUseCase<R> {
    pub async fn add(&self, issue: Issue) -> anyhow::Result<String> {
        self.repositories
            .github_issue_repository()
            .add(issue)
            .await
            .context("GitHubにIssueを作成できませんでした")
    }
}
