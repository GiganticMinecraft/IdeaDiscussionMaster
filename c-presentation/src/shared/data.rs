use super::global::{GlobalAgenda, GlobalAgendas};
use c_domain::repository::{AgendaRepository, RecordRepository};
use c_infra::repository::RedmineRepositoryImpl;

use std::{collections::HashSet, sync::Arc};

pub struct Repos {
    pub agenda: Arc<dyn AgendaRepository + Sync + Send>,
    pub record: Arc<dyn RecordRepository + Sync + Send>,
}

impl Repos {
    pub fn new(redmine_url: String) -> Self {
        Self {
            agenda: Arc::new(RedmineRepositoryImpl::<c_domain::Agenda>::new(
                redmine_url.clone(),
            )),
            record: Arc::new(RedmineRepositoryImpl::<c_domain::Record>::new(redmine_url)),
        }
    }
}

pub struct Data {
    pub repos: Repos,
    pub agendas: GlobalAgendas,
}

impl Data {
    pub fn new(redmine_url: String, agendas: HashSet<GlobalAgenda>) -> Self {
        Self {
            repos: Repos::new(redmine_url),
            agendas: GlobalAgendas::new(agendas),
        }
    }
}
