use serenity::framework::standard::CommandResult;
use std::fmt;
use strum::EnumMessage;

#[derive(Debug)]
pub enum Error {
    Reqwest(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, EnumMessage)]
pub enum DiscussionError {
    #[strum(message = "引数が指定されていません。")]
    ArgIsNotSpecified(SpecifiedArgs),
    #[strum(message = "チケットが存在しません。")]
    TicketIsNotFound,
    #[strum(message = "VCに参加されていません。")]
    VcIsNotJoined,
    #[strum(
        message = "Redmineへのアクセス中に不明なエラーが発生しました。管理者に連絡してください。"
    )]
    UnknownError(Error),
}

#[derive(Debug, EnumMessage)]
pub enum SpecifiedArgs {
    #[strum(message = "チケット番号")]
    TicketNumber,
    #[strum(message = "チケットのステータス")]
    TicketStatus,
    #[strum(message = "議事録の日付")]
    RecordDate,
}

impl DiscussionError {
    pub fn get_msg(&self) -> String {
        let msg = self
            .get_message()
            .unwrap_or("エラーが発生しました。")
            .to_string();
        if let Self::ArgIsNotSpecified(arg) = self {
            format!("{}({})", msg, arg.ja())
        } else {
            msg
        }
    }
}

impl SpecifiedArgs {
    pub fn ja(&self) -> String {
        self.get_message()
            .unwrap_or("指定されるべき引数")
            .to_string()
    }
}

impl From<reqwest::Error> for DiscussionError {
    fn from(err: reqwest::Error) -> DiscussionError {
        Self::UnknownError(Error::Reqwest(err.to_string()))
    }
}

impl From<Box<(dyn std::error::Error)>> for DiscussionError {
    fn from(err: Box<(dyn std::error::Error)>) -> DiscussionError {
        Self::UnknownError(Error::Other(err.to_string()))
    }
}

impl From<DiscussionError> for CommandResult {
    fn from(err: DiscussionError) -> CommandResult {
        Err(err.to_string().into())
    }
}

impl ToString for DiscussionError {
    fn to_string(&self) -> String {
        match self {
            Self::UnknownError(err) => format!("{:?}\n{:?}", self.get_msg(), err),
            _ => self.get_msg(),
        }
    }
}
