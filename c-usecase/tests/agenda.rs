mod agenda_fixture;
use agenda_fixture::AgendaFixture;

use c_domain::{id::AgendaId, repository::MockAgendaRepository, Agenda};
use c_usecase::AgendaUseCase;

use std::sync::Arc;

#[tokio::test]
async fn test_with_err_repo() {
    let mut repo = MockAgendaRepository::new();
    repo.expect_find()
        .returning(|_| Err(anyhow::anyhow!("There are no agendas")));
    let use_case = AgendaUseCase::new(Arc::new(repo));

    assert!(use_case.find(AgendaId::new(1)).await.is_err());
    assert!(use_case.find(AgendaId::new(2)).await.is_err());
    assert!(use_case.find(AgendaId::new(3)).await.is_err());
}

// TODO: add test of #in_progress or changing status
#[tokio::test]
async fn test_with_fixtures() {
    let mut repo = MockAgendaRepository::new();
    repo.expect_find().returning(|id| {
        Agenda::all_fixtures()
            .into_iter()
            .find(|agenda| agenda.id == id)
            .ok_or_else(|| anyhow::anyhow!("The agenda you want does not exist"))
    });
    repo.expect_save().returning(|_| Ok(()));
    let use_case = AgendaUseCase::new(Arc::new(repo));

    assert!(use_case.find(AgendaId::new(1)).await.is_ok());
    assert!(use_case.find(AgendaId::new(2)).await.is_ok());
    assert!(use_case.find(AgendaId::new(10)).await.is_err());

    assert!(use_case.find_new(AgendaId::new(1)).await.is_ok());
    assert!(use_case.find_new(AgendaId::new(2)).await.is_err());
    assert!(use_case.find_new(AgendaId::new(3)).await.is_err());
    assert!(use_case.find_new(AgendaId::new(10)).await.is_err());
}
