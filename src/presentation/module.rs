use crate::{
    infrastructure::{
        module::{RepositoryModule, RepositoryModuleExt},
        persistence::*,
    },
    usecase::*,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Module {
    agenda_usecase: AgendaUseCase<RepositoryModule>,
    record_usecase: RecordUseCase<RepositoryModule>,
    gh_issue_usecase: GitHubIssueUseCase<RepositoryModule>,
}

impl Module {
    pub async fn new() -> Self {
        let module = Arc::new(RepositoryModule::new(Redmine::new(), GitHub::new().await));

        let agenda_usecase = AgendaUseCase::new(module.clone());
        let record_usecase = RecordUseCase::new(module.clone());
        let gh_issue_usecase = GitHubIssueUseCase::new(module);

        Self {
            agenda_usecase,
            record_usecase,
            gh_issue_usecase,
        }
    }
}

pub trait ModuleExt {
    type RepoModule: RepositoryModuleExt;

    fn agenda_usecase(&self) -> &AgendaUseCase<Self::RepoModule>;
    fn record_usecase(&self) -> &RecordUseCase<Self::RepoModule>;
    fn gh_issue_usecase(&self) -> &GitHubIssueUseCase<Self::RepoModule>;
}

impl ModuleExt for Module {
    type RepoModule = RepositoryModule;

    fn agenda_usecase(&self) -> &AgendaUseCase<Self::RepoModule> {
        &self.agenda_usecase
    }

    fn record_usecase(&self) -> &RecordUseCase<Self::RepoModule> {
        &self.record_usecase
    }

    fn gh_issue_usecase(&self) -> &GitHubIssueUseCase<Self::RepoModule> {
        &self.gh_issue_usecase
    }
}
