use derive_new::new;
use serenity::model::interactions::application_command::ApplicationCommandOptionType;
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

#[allow(dead_code)]
#[derive(new, Debug)]
pub struct CommandInfo {
    name: String,
    description: String,
}

#[derive(Error, Debug)]
pub enum CommandBuilderError {
    #[error("選択肢の型が一致していません。: {command:?} -> {choice_name} {choice:?}")]
    ChoiceAndOptionTypeMisMatch {
        command: CommandInfo,
        choice_name: String,
        choice: ApplicationCommandOptionType,
    },
}
