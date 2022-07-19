use crate::{
    global,
    module::{Module, ModuleExt},
    shared::{
        command::{CommandResult, ExecutorArgs, InteractionResponse},
        ext::{CommandExt, CreateEmbedExt, IdExt},
    },
};
use crate_domain::{
    error::MyError, github::Issue as GHIssue, id::IssueId, redmine::Note, status::AgendaStatus,
};
use crate_usecase::model::{AgendaDto, DtoExt};

use anyhow::{ensure, Context};
use futures::stream::{self, StreamExt};
use itertools::Itertools;
use log::{debug, info};
use serenity::builder::CreateEmbed;

async fn fetch_agendas(
    module: &Module,
    id: IssueId,
) -> (IssueId, Result<AgendaDto, anyhow::Error>) {
    (id, module.agenda_usecase().find(id).await)
}

fn refine_all_related_ideas(
    ideas: Vec<IssueId>,
    relations: &[IssueId],
) -> anyhow::Result<Vec<IssueId>> {
    let related = ideas
        .clone()
        .into_iter()
        .filter(|id| relations.contains(id))
        .collect_vec();
    let not_related = ideas
        .iter()
        .filter(|id| !related.contains(id))
        .collect_vec();
    ensure!(
        not_related.is_empty(),
        "議事録に関連付けられていないチケットがあります。:{:?}",
        not_related.iter().map(|id| id.formatted()).collect_vec()
    );

    Ok(related)
}

fn refine_all_fetched_idea(
    ideas: Vec<(IssueId, Result<AgendaDto, anyhow::Error>)>,
) -> anyhow::Result<Vec<AgendaDto>> {
    let succeeded = ideas
        .iter()
        .filter(|(_, res)| res.is_ok())
        .map(|(_, res)| res.as_ref().unwrap().to_owned())
        .collect_vec();
    let failed = ideas
        .iter()
        .filter(|(id, _)| !succeeded.iter().map(|v| v.id).contains(id))
        .map(|(id, res)| (id, res.as_ref().err().unwrap()))
        .collect_vec();
    ensure!(
        failed.is_empty(),
        "詳細を取得できない議題があります。:{:?}",
        failed
            .iter()
            .map(|(id, err)| format!("{} {:?}", id.formatted(), err))
            .collect_vec()
    );

    Ok(succeeded)
}

fn refine_all_approved_ideas(ideas: Vec<AgendaDto>) -> anyhow::Result<Vec<AgendaDto>> {
    let not_approved = ideas
        .iter()
        .filter(|v| v.status != AgendaStatus::Approved)
        .collect_vec();
    let approved = ideas
        .iter()
        .filter(|v| !not_approved.contains(v))
        .map(|dto| dto.to_owned())
        .collect_vec();
    ensure!(
        not_approved.is_empty(),
        "承認されていない議題があります。:{:?}",
        not_approved.iter().map(|v| v.id.formatted()).collect_vec()
    );

    Ok(approved)
}

/// 指定された議題を連結した文字列が当該議事録の承認された議題として正しいかどうかを確認する
///
/// すべての議題が以下の条件を満たす必要がある
/// * u16にパースできる
/// * 議事録に関連付けられている
/// * ステータスが承認である
///
/// ## 引数
///
/// * `idea_args` - 議題のチケット番号をスペース区切りでつなげた文字列
/// * `relations` - 議事録の関連チケットID
/// * `module` - ユースケースを解決するModule
async fn fetch_ideas(
    idea_args: String,
    relations: &[IssueId],
    module: &Module,
) -> anyhow::Result<Vec<AgendaDto>> {
    debug!("議題文字列: {:?}", idea_args);
    let ideas = idea_args
        .split(' ')
        .filter_map(|str| str.parse::<u16>().ok())
        .map(IssueId::new)
        .collect_vec();
    ensure!(
        !ideas.is_empty(),
        "指定された文字列を議題のリストとして認識できません。"
    );

    let related = refine_all_related_ideas(ideas, relations)?;

    let fetch_agenda_results: Vec<_> = stream::iter(related)
        .then(|id| fetch_agendas(module, id))
        .collect()
        .await;
    let fetched = refine_all_fetched_idea(fetch_agenda_results)?;

    refine_all_approved_ideas(fetched)
}

async fn create_gh_issue(
    id: IssueId,
    issue: GHIssue,
    module: &Module,
) -> (IssueId, anyhow::Result<String>) {
    (id, module.gh_issue_usecase().add(issue).await)
}

fn group_github_issues(
    issues: Vec<(IssueId, anyhow::Result<String, anyhow::Error>)>,
) -> (Vec<(IssueId, String)>, Vec<(IssueId, anyhow::Error)>) {
    let succeeded = issues
        .iter()
        .filter(|(_, res)| res.is_ok())
        .map(|(id, res)| (id.to_owned(), res.as_ref().unwrap().to_owned()))
        .collect_vec();
    let failed = issues
        .into_iter()
        .filter(|(_, res)| res.is_err())
        .map(|(id, res)| (id, res.err().unwrap()))
        .collect_vec();

    (succeeded, failed)
}

async fn add_redmine_notes(
    id: IssueId,
    note: Note,
    module: &Module,
) -> (IssueId, anyhow::Result<()>) {
    (id, module.record_usecase().add_note(id, note).await)
}

fn group_redmine_notes(
    notes: Vec<(IssueId, anyhow::Result<()>)>,
) -> (Vec<IssueId>, Vec<(IssueId, anyhow::Error)>) {
    let succeeded = notes
        .iter()
        .filter(|(_, res)| res.is_ok())
        .map(|(id, _)| id.to_owned())
        .collect_vec();
    let failed = notes
        .into_iter()
        .filter(|(_, res)| res.is_err())
        .map(|(id, res)| (id, res.err().unwrap()))
        .collect_vec();

    (succeeded, failed)
}

fn create_failures_embed(failed: &[(IssueId, anyhow::Error)], record_id: &IssueId) -> CreateEmbed {
    let contents = failed
        .iter()
        .map(|(id, err)| format!("{}\n{:?}", id.formatted(), err))
        .join("\n\n");

    CreateEmbed::default()
        .custom_default(record_id)
        .description(contents)
        .failure_color()
        .to_owned()
}

#[allow(clippy::type_complexity)]
pub async fn issue((map, ctx, interaction): ExecutorArgs) -> CommandResult {
    let module = global::module::get();

    info!("Start /create issue");

    // 議事録のIDを取得
    let record_id: u16 = map
        .get("record_issue_number")
        .ok_or_else(|| MyError::ArgIsNotFound("record_issue_number".to_string()))?
        .to_owned()
        .try_into()?;
    let record = module
        .record_usecase()
        .find(IssueId::new(record_id))
        .await
        .with_context(|| format!("議事録の取得中にエラーが発生しました。: #{:?}", record_id))?;

    // Issueを作成するアイデアを取得
    let ideas: String = map
        .get("idea_issue_numbers")
        .ok_or_else(|| MyError::ArgIsNotFound("idea_issue_numbers".to_string()))?
        .to_owned()
        .try_into()?;
    let ideas = fetch_ideas(ideas, &record.relations, &module).await?;

    info!("Create GitHub issues");
    // GitHubにIssueを作成
    let gh_issues = ideas
        .iter()
        .map(|idea| {
            let title = format!("Redmine Idea {}", idea.id.formatted());
            let content = format!(
                "{}\n[{}]({})にて承認されたアイデア。",
                idea.url(),
                record.discussion_title(),
                record.url()
            );

            (
                idea.id,
                GHIssue::new(
                    title,
                    content,
                    vec!["Tracked: Redmine", "Status/Idea: Accepted✅"]
                        .into_iter()
                        .map(|str| str.to_string())
                        .collect_vec(),
                ),
            )
        })
        .collect_vec();
    let github_result: Vec<_> = stream::iter(gh_issues)
        .then(|(id, issue)| create_gh_issue(id, issue, &module))
        .collect()
        .await;
    let (gh_issues, failed_gh_issues) = group_github_issues(github_result);
    let github_failures_embed = create_failures_embed(&failed_gh_issues, &record.id)
        .title("GitHubにIssueを起票できなかった議題があります")
        .to_owned();

    info!("Add Redmine notes");
    // RedmineにGitHubのIssueのURLを注記
    let redmine_notes = gh_issues
        .iter()
        .map(|(id, gh_issue_url)| {
            (
                *id,
                Note::new(
                    vec![
                        "GitHubにIssueを作成しました。以下URLより確認できます。",
                        gh_issue_url,
                    ]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect_vec(),
                ),
            )
        })
        .collect_vec();
    let redmine_result: Vec<_> = stream::iter(redmine_notes)
        .then(|(id, note)| add_redmine_notes(id, note, &module))
        .collect()
        .await;
    let (redmine_notes, failed_redmine_notes) = group_redmine_notes(redmine_result);
    let redmine_failures_embed = create_failures_embed(&failed_redmine_notes, &record.id)
        .title("Redmineに注記できなかった議題があります")
        .to_owned();

    let success_embed = CreateEmbed::default()
        .custom_default(&record.id)
        .title("GitHubへの起票とRedmineへの注記をどちらも完了した議題は以下の通りです")
        .description(redmine_notes.iter().map(|id| id.formatted()).join(", "))
        .success_color()
        .to_owned();

    let results = {
        let mut res: Vec<CreateEmbed> = vec![];
        if !redmine_notes.is_empty() {
            res.push(success_embed);
        }
        if !failed_gh_issues.is_empty() {
            res.push(github_failures_embed)
        }
        if !failed_redmine_notes.is_empty() {
            res.push(redmine_failures_embed)
        }

        res
    };

    // 結果を送信する
    interaction
        .send(&ctx.http, InteractionResponse::Embeds(results))
        .await
        .map(|_| ())
}
