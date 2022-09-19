use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("このコマンドを実行するには会議が開始されている必要があります")]
    DiscussionHasBeenStarted,
    #[error("現在進行中の議題はありません")]
    AgendaIsNotFound,
}
