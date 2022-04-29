use crate::{
    domain::{gh_issue::Issue, repository::GitHubIssueRepository},
    infrastructure::module::RepositoryModuleExt,
};
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct GitHubIssueUseCase<R: RepositoryModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoryModuleExt> GitHubIssueUseCase<R> {
    pub async fn add(&self, issue: Issue) -> anyhow::Result<String> {
        self.repositories.github_issue_repository().add(issue).await
    }
}
