use super::RedmineRepositoryImpl;
use crate::model::redmine::{CreateNote, UpdateAgenda};
use c_domain::redmine::{
    model::{id::AgendaId, Agenda, Note},
    repository::AgendaRepository,
};

use anyhow::ensure;
use async_trait::async_trait;

#[async_trait]
impl AgendaRepository for RedmineRepositoryImpl<Agenda> {
    async fn find(&self, id: &AgendaId) -> anyhow::Result<Agenda> {
        let res = self.client.get(id.0).await?;
        ensure!(
            res.issue.is_idea_ticket(),
            "チケットはアイデア提案のものではありません"
        );

        Ok(res.issue.into())
    }

    async fn save(&self, agenda: Agenda) -> anyhow::Result<()> {
        let agenda_id = agenda.id.clone();
        let value = UpdateAgenda::new(agenda.into());
        let _ = self.client.put(agenda_id.0, &value).await?;

        Ok(())
    }

    async fn add_note(&self, id: &AgendaId, note: Note) -> anyhow::Result<()> {
        let value = CreateNote::new(note.into());
        let _ = self.client.put(id.0, &value).await?;

        Ok(())
    }
}
