use crate::shared::global::{
    GlobalCurrentAgendaId, GlobalRecordId, GlobalVcId, GlobalVoteMessageId,
};
use c_domain::{
    github::{model::Issue, repository::GitHubIssueRepository},
    redmine::{
        model::{Agenda, Record},
        repository::{AgendaRepository, RecordRepository},
    },
};
use c_infra::{
    github::repository::GitHubRepositoryImpl, redmine::repository::RedmineRepositoryImpl,
};
use c_usecase::{
    github::IssueUseCase,
    redmine::{AgendaUseCase, RecordUseCase},
};

use derive_new::new;
use std::sync::Arc;

pub struct Repos {
    pub agenda: Arc<dyn AgendaRepository + Sync + Send>,
    pub record: Arc<dyn RecordRepository + Sync + Send>,
    pub issue: Arc<dyn GitHubIssueRepository + Sync + Send>,
}

impl Repos {
    pub async fn new(redmine_url: String) -> Self {
        Self {
            agenda: Arc::new(RedmineRepositoryImpl::<Agenda>::new(redmine_url.clone())),
            record: Arc::new(RedmineRepositoryImpl::<Record>::new(redmine_url)),
            issue: Arc::new(GitHubRepositoryImpl::<Issue>::new().await),
        }
    }
}

#[derive(new)]
pub struct UseCases {
    pub agenda: AgendaUseCase,
    pub record: RecordUseCase,
    pub issue: IssueUseCase,
}

pub struct Data {
    pub use_cases: UseCases,
    pub vc_id: GlobalVcId,
    pub record_id: GlobalRecordId,
    pub current_agenda_id: GlobalCurrentAgendaId,
    pub vote_message_id: GlobalVoteMessageId,
}

impl Data {
    pub async fn new(redmine_url: String) -> Self {
        let repos = Repos::new(redmine_url).await;
        let use_cases = UseCases::new(
            AgendaUseCase::new(repos.agenda),
            RecordUseCase::new(repos.record),
            IssueUseCase::new(repos.issue),
        );

        Self {
            use_cases,
            vc_id: GlobalVcId::new(),
            record_id: GlobalRecordId::new(),
            current_agenda_id: GlobalCurrentAgendaId::new(),
            vote_message_id: GlobalVoteMessageId::new(),
        }
    }
}
