use crate::domains::{
    client::{GitHubClient, RedmineClient},
    custom_error::{DiscussionError, SpecifiedArgs},
    github::CreateIssueResponse,
    redmine,
    status::AgendaStatus,
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
        .filter(|agenda| {
            AgendaStatus::from_ja(&agenda.status.name)
                .map(|status| status == AgendaStatus::Approved)
                .unwrap_or(false)
        })
        .collect_vec();

    let record_url = format!("{}/issues/{}", redmine::REDMINE_URL, record.id);
    // 「第◯回」の表記を議事録のタイトルから抜き出す
    let record_number = Regex::new("第[0-9]{1,9}回")
        .unwrap()
        .captures(&record.subject)
        .and_then(|cap| cap.get(0))
        .map_or("", |m| m.as_str());

    // GitHubのIssueを作っていく
    let mut github_issue_results: Vec<(u16, Option<String>)> = Vec::new();
    for agenda in agendas {
        let res = make_github_issue(&agenda.id, &record_url, record_number).await;
        github_issue_results.push((agenda.id, res.ok()));
    }
    let (issued, unissued): (Vec<_>, Vec<_>) = github_issue_results
        .iter()
        .partition(|(_, url)| url.is_some());
    let issued = issued
        .iter()
        .map(|(id, url)| (id, url.as_ref().unwrap()))
        .collect_vec();
    let unissued = unissued.iter().map(|tup| tup.0).collect_vec();

    // GitHubにIssueを作成できたもののみ、RedmineにそのIssueのURLを記載する
    let mut redmine_issue_results: Vec<Option<u16>> = Vec::new();
    for (agenda_id, issue_url) in &issued {
        let contents = format!(
            "GitHubにIssueを作成しました。以下URLより確認できます。\n{}",
            issue_url
        );
        let result = redmine_client
            .add_comments(**agenda_id, vec![contents])
            .await
            .ok()
            .map(|_| **agenda_id);
        redmine_issue_results.push(result);
    }
    let commented = redmine_issue_results.iter().flatten().collect_vec();
    let uncommented = issued
        .into_iter()
        .map(|tup| tup.0)
        .filter(|id| !commented.iter().contains(id))
        .collect_vec();

    let result_messages = vec![
        "それぞれの議題につき",
        "GitHubにIssueを作成できなかったものは以下",
        &unissued.iter().join(", "),
        "Redmineにコメントを記載できなかったものは以下",
        &uncommented.iter().join(", "),
        "以上に挙げたもの以外は正常に処理を終了しました。",
    ]
    .iter()
    .join("\n");

    let _ = message.reply(&ctx.http, result_messages).await;

    Ok(())
}

async fn make_github_issue(
    agenda_id: &u16,
    record_url: &str,
    record_number: &str,
) -> Result<String, DiscussionError> {
    let title = format!("Redmine Idea #{}", agenda_id);
    let agenda_url = format!("{}/issues/{}", redmine::REDMINE_URL, agenda_id);
    let content = format!(
        "{}\n[{}アイデア会議]({})にて承認されたアイデア。",
        agenda_url, record_number, record_url
    );

    Ok(GitHubClient::new()
        .await
        .create_issue(
            &title,
            &content,
            vec!["Tracked: Redmine", "Status/Idea: Accepted✅"],
        )
        .await?
        .json::<CreateIssueResponse>()
        .await?
        .html_url)
}
