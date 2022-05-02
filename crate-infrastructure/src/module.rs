use super::{persistence::*, repository::*};
use crate_domain::{github::Issue, redmine::*, repository::RepositoryModuleExt};

type AgendaRepoImpl = RedminePersistenceImpl<Agenda>;
type RecordRepoImpl = RedminePersistenceImpl<Record>;
type GHIssueRepoImpl = GitHubPersistenceImpl<Issue>;

#[derive(Clone)]
pub struct RepositoryModule {
    agenda_repository: AgendaRepoImpl,
    record_repository: RecordRepoImpl,
    github_issue_repository: GHIssueRepoImpl,
}

impl RepositoryModule {
    pub fn new(redmine_client: Redmine, github_client: GitHub) -> Self {
        let agenda_repository = RedminePersistenceImpl::new(redmine_client.clone());
        let record_repository = RedminePersistenceImpl::new(redmine_client);
        let github_issue_repository = GitHubPersistenceImpl::new(github_client);

        Self {
            agenda_repository,
            record_repository,
            github_issue_repository,
        }
    }
}

impl RepositoryModuleExt for RepositoryModule {
    type AgendaRepo = AgendaRepoImpl;
    type RecordRepo = RecordRepoImpl;
    type GHIssueRepo = GHIssueRepoImpl;

    fn agenda_repository(&self) -> &Self::AgendaRepo {
        &self.agenda_repository
    }
    fn record_repository(&self) -> &Self::RecordRepo {
        &self.record_repository
    }
    fn github_issue_repository(&self) -> &Self::GHIssueRepo {
        &self.github_issue_repository
    }
}
