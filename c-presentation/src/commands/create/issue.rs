use super::shared::parse_string_as_agenda_ids;
use crate::{
    commands::{CommandResult, Context},
    shared::ext::{CreateEmbedExt, SortAgendasExt, UseFormattedId},
};
use c_domain::redmine::model::id::{AgendaId, RecordId};
use c_usecase::{github::model::CreateIssueParam, redmine::model::CreateNoteParam};

use anyhow::ensure;
use itertools::Itertools;
use log::info;
use poise::{futures_util::future, serenity_prelude::CreateEmbed};

type ErrAgendas = Vec<(AgendaId, anyhow::Error)>;

fn group_results<T>(results: Vec<(AgendaId, anyhow::Result<T>)>) -> (Vec<(AgendaId, T)>, ErrAgendas)
where
    T: Clone,
{
    let succeeded = results
        .iter()
        .filter(|(_, res)| res.is_ok())
        .map(|(id, res)| (id.to_owned(), res.as_ref().unwrap().clone()))
        .collect_vec();
    let failed = results
        .into_iter()
        .filter(|(_, res)| res.is_err())
        .map(|(id, res)| (id, res.err().unwrap()))
        .collect_vec();

    (succeeded, failed)
}

fn create_failures_embed<'a>(
    embed: &'a mut CreateEmbed,
    errs: &ErrAgendas,
    record_id: &RecordId,
) -> &'a mut CreateEmbed {
    let contents = errs
        .iter()
        .map(|(id, err)| format!("{}\n{:?}", id.formatted(), err))
        .join("\n\n");

    embed
        .custom_default(record_id)
        .description(contents)
        .failure_color()
}

/// SeichiAssistのGitHubにIssueを追加します
#[poise::command(slash_command)]
pub async fn issue(
    ctx: Context<'_>,
    #[description = "Issueを作成する議事録の番号"]
    #[min = 1]
    record_issue_number: u16,
    #[description = "Issueを作成しない議題のチケット番号群(半角スペース区切り)"]
    excluded_idea_issue_numbers: String,
) -> CommandResult {
    let record_id = RecordId::new(record_issue_number);
    let record = ctx.data().use_cases.record.find(&record_id).await?;
    info!("record_id: {}", record_id.formatted());
    let excluded_idea_ids = parse_string_as_agenda_ids(excluded_idea_issue_numbers)?;
    let agendas = ctx
        .data()
        .use_cases
        .agenda
        .list(
            &record
                .relations
                .clone()
                .into_iter()
                .map(AgendaId::new)
                .filter(|id| !excluded_idea_ids.contains(id))
                .collect_vec(),
        )
        .await
        .into_iter()
        .filter(|dto| dto.status.is_approved())
        .collect_vec()
        .sort_by_id();
    info!(
        "selected approved ideas: {}",
        agendas
            .iter()
            .map(|dto| AgendaId::new(dto.id).formatted())
            .join(", ")
    );

    info!("Create GitHub issues");
    let create_gh_issue_params = agendas
        .iter()
        .map(|agenda| {
            let title = format!("Redmine Idea {}", AgendaId::new(agenda.id).formatted());
            let content = format!(
                "{}\n[{}]({})にて承認されたアイデア。",
                agenda.url(),
                record.discussion_title(),
                record.url()
            );

            CreateIssueParam::new(
                title,
                content,
                vec!["Tracked: Redmine", "Status/Idea: Accepted✅"]
                    .into_iter()
                    .map(|str| str.to_string())
                    .collect_vec(),
            )
        })
        .collect_vec();
    let github_issue_results = future::join_all(
        create_gh_issue_params
            .into_iter()
            .map(|param| async move { ctx.data().use_cases.issue.add(param).await }),
    )
    .await;
    let github_issue_results = agendas
        .into_iter()
        .map(|dto| AgendaId::new(dto.id))
        .zip(github_issue_results.into_iter())
        .collect_vec();
    let (gh_issues, err_gh_issues) = group_results::<String>(github_issue_results);

    info!("Add Redmine notes");
    let create_redmine_notes = gh_issues
        .into_iter()
        .map(|(id, gh_issue_url)| {
            (
                id,
                CreateNoteParam::from_multi_line_string(vec![
                    "GitHubにIssueを作成しました。以下URLより確認できます。".to_string(),
                    gh_issue_url,
                ]),
            )
        })
        .collect_vec();
    let redmine_note_results =
        future::join_all(create_redmine_notes.iter().map(|(id, param)| async move {
            ctx.data()
                .use_cases
                .agenda
                .add_note(id, param.clone())
                .await
        }))
        .await;
    let redmine_note_results = create_redmine_notes
        .into_iter()
        .map(|(id, _)| id)
        .zip(redmine_note_results.into_iter())
        .collect_vec();
    let (redmine_notes, err_redmine_notes) = group_results::<()>(redmine_note_results);

    ctx.send(|r| {
        if !redmine_notes.is_empty() {
            r.embed(|e| {
                e.custom_default(&record_id)
                    .title("GitHubへの起票とRedmineへの注記をどちらも完了した議題は以下の通りです")
                    .description(
                        redmine_notes
                            .iter()
                            .map(|(id, _)| id.formatted())
                            .join(", "),
                    )
                    .success_color()
            });
        }
        if !err_gh_issues.is_empty() {
            r.embed(|e| {
                create_failures_embed(e, &err_gh_issues, &record_id)
                    .title("GitHubにIssueを起票できなかった議題があります")
            });
        }
        if !err_redmine_notes.is_empty() {
            r.embed(|e| {
                create_failures_embed(e, &err_redmine_notes, &record_id)
                    .title("Redmineに注記をできなかった議題があります")
            });
        }

        r
    })
    .await?;

    Ok(())
}
