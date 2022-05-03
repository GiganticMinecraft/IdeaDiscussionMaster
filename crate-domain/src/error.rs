use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("引数が指定されていません。")]
    ArgIsNotSpecified,
    #[error("チケットが存在しません。")]
    TicketIsNotFound,
    #[error("チケットはアイデア提案用プロジェクトのアイデアではありません。")]
    TicketIsNotIdea,
    #[error("チケットはアイデア提案用プロジェクトの未議論アイデアではありません。")]
    TicketIsNotUndoneIdea,
    #[error("チケットはアイデア会議議事録プロジェクトの議事録ではありません。")]
    TicketIsNotIdeaDiscussionRecord,
    #[error("チケットはアイデア会議議事録プロジェクトの未終了議事録ではありません。")]
    TicketIsNotUndoneIdeaDiscussionRecord,
    #[error("チケットのステータスは終了ステータスとして適切なものではありません。")]
    TicketStatusIsNotDone,
    #[error("VCに参加されていないようです。")]
    IsNotJoinedInVC,
    #[error("予期しないステータスが指定されています。:({0} {1})")]
    TicketHasUnexpectedStatus(u16, String),
}

#[derive(Error, Debug)]
pub enum CommandBuilderError {
    #[error("入力を処理する関数が指定されていません。: {name} {description}")]
    ExecutorIsNotDefined { name: String, description: String },
}
