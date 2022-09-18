use crate::github::model::CreateIssueParam;
use c_domain::github::repository::GitHubIssueRepository;

use derive_new::new;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct IssueUseCase {
    repo: Arc<dyn GitHubIssueRepository + Sync + Send>,
}

impl IssueUseCase {
    pub async fn add(&self, param: CreateIssueParam) -> anyhow::Result<String> {
        self.repo.add(param.into()).await
    }
}
