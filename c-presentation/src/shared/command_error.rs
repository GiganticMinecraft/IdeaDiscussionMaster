use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("このコマンドを実行するには会議が開始されている必要があります")]
    DiscussionHasBeenStarted,
}
