use crate::domains::{
    custom_error::{DiscussionError, SpecifiedArgs},
    redmine, GitHubClient, RedmineClient,
};
use futures::stream::{self, StreamExt};
use itertools::Itertools;
use regex::Regex;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[aliases("agi", "ghissue", "ghi")]
#[usage = "[議事録のチケット番号] [..GitHubに起票する議題のチケット番号（半角スペースで区切る）]"]
#[description = "指定した議事録の指定した議題チケットについてSeichiAssistのリポジトリでIssueを作成します。存在しない議題は無視されます。"]
async fn add_github_issue(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
    let record_id = match args.single::<u16>() {
        Ok(id) if id > 0 => id,
        _ => {
            return DiscussionError::ArgIsNotSpecified(SpecifiedArgs::TicketNumber).into();
        }
    };
    let redmine_client = RedmineClient::new();
    let record = match redmine_client.fetch_issue_with_relations(record_id).await {
        Ok(record) => {
            if record.is_idea_discussion_record() {
                record
            } else {
                return DiscussionError::ArgIsNotSpecified(SpecifiedArgs::TicketNumber).into();
            }
        }
        Err(err) => return err.into(),
    };

    // 第2引数以降に受け取った議題のチケット番号群を議事録の関連チケット群と比較し、ヒットしたもののみフィルタする
    let agendas = args
        .iter::<u16>()
        .filter_map(|num| num.ok())
        .filter(|num| record.relations().contains(num))
        .collect_vec();
    if agendas.is_empty() {
        return DiscussionError::ArgIsNotSpecified(SpecifiedArgs::TicketNumber).into();
    }
    let agendas = stream::iter(agendas)
        .then(|id| redmine_client.fetch_issue(id))
        .collect::<Vec<_>>()
        .await;
    let agendas = agendas
        .iter()
        .filter_map(|res| res.as_ref().ok())
        .collect_vec();

    let record_url = format!("{}/issues/{}", redmine::REDMINE_URL, record.id);
    // 「第◯回」の表記を抜き出す
    let record_number = Regex::new("第[0-9]{1,9}回")
        .unwrap()
        .captures(&record.subject)
        .and_then(|cap| cap.get(0))
        .map_or("", |m| m.as_str());
    // GitHubのIssueを作っていく
    for agenda in agendas {
        let title = format!("Redmine Idea #{}", agenda.id);
        let agenda_url = format!("{}/issues/{}", redmine::REDMINE_URL, agenda.id);
        let content = format!(
            "{}\n[{}アイデア会議]({})にて承認されたアイデア。",
            agenda_url, record_number, record_url
        );

        let _ = GitHubClient::new()
            .create_issue(
                &title,
                &content,
                vec!["Tracked: Redmine", "Status/Idea: Accepted✅"],
            )
            .await;
    }

    let _ = message
        .reply(
            &ctx.http,
            "すべての指定されたチケットについてIssueを作成しました。",
        )
        .await;

    Ok(())
}
