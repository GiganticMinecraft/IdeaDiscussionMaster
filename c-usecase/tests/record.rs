mod record_fixture;
use record_fixture::RecordFixture;

use c_domain::{id::RecordId, repository::MockRecordRepository, status::RecordStatus, Record};
use c_usecase::RecordUseCase;

use std::sync::Arc;

#[tokio::test]
async fn test_with_err_repo() {
    let mut repo = MockRecordRepository::new();
    repo.expect_find()
        .returning(|_| Err(anyhow::anyhow!("There are no records")));
    let use_case = RecordUseCase::new(Arc::new(repo));

    assert!(use_case.find(&RecordId::new(1)).await.is_err());
    assert!(use_case.find(&RecordId::new(2)).await.is_err());
    assert!(use_case.find(&RecordId::new(3)).await.is_err());
}

// TODO: impl #find_latest_new
// TODO: 分割しろ
#[tokio::test]
async fn test_with_fixtures() {
    let mut repo = MockRecordRepository::new();
    repo.expect_find().returning(|id| {
        Record::all_fixtures()
            .into_iter()
            .find(|record| &record.id == id)
            .ok_or_else(|| anyhow::anyhow!("The record you want does not exist"))
    });
    repo.expect_list()
        .withf(|_, statuses| !statuses.is_empty())
        .returning(|limit, statuses| {
            Ok(Record::all_fixtures()
                .into_iter()
                .filter(|record| statuses.contains(&record.status))
                .take(limit.unwrap_or(20).into())
                .collect())
        });
    let use_case = RecordUseCase::new(Arc::new(repo));

    assert!(use_case.find(&RecordId::new(1)).await.is_ok());
    assert!(use_case.find(&RecordId::new(2)).await.is_ok());
    assert!(use_case.find(&RecordId::new(10)).await.is_err());

    assert!(use_case.find_new(&RecordId::new(1)).await.is_ok());
    assert!(use_case.find_new(&RecordId::new(2)).await.is_err());
    assert!(use_case.find_new(&RecordId::new(3)).await.is_err());
    assert!(use_case.find_new(&RecordId::new(10)).await.is_err());

    assert_eq!(
        use_case.list(None, vec![RecordStatus::New]).await.unwrap(),
        vec![Record::new1().into()]
    );
    assert_eq!(
        use_case
            .list(None, vec![RecordStatus::New, RecordStatus::Closed])
            .await
            .unwrap(),
        vec![
            Record::new1().into(),
            Record::closed1().into(),
            Record::closed2().into(),
            Record::closed3().into()
        ]
    );
    assert_eq!(
        use_case
            .list(Some(2), vec![RecordStatus::New, RecordStatus::Closed])
            .await
            .unwrap(),
        vec![Record::new1().into(), Record::closed1().into()]
    );

    assert_eq!(
        use_case.find_latest_closed().await.unwrap(),
        Record::closed1().into()
    );
}
