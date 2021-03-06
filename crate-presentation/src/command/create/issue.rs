use crate::{
    global,
    module::{Module, ModuleExt},
    shared::{
        command::{CommandResult, ExecutorArgs, InteractionResponse},
        ext::{CommandExt, CreateEmbedExt, IdExt},
        issue_id_array_parser::refine_all_agendas,
    },
};
use crate_domain::{
    error::MyError, github::Issue as GHIssue, id::IssueId, redmine::Note, status::AgendaStatus,
};
use crate_usecase::model::DtoExt;

use anyhow::Context;
use futures::stream::{self, StreamExt};
use itertools::Itertools;
use log::info;
use serenity::builder::CreateEmbed;

async fn create_gh_issue(
    id: IssueId,
    issue: GHIssue,
    module: &Module,
) -> (IssueId, anyhow::Result<String>) {
    (id, module.gh_issue_usecase().add(issue).await)
}

type FailedIssues = Vec<(IssueId, anyhow::Error)>;

fn group_github_issues(
    issues: Vec<(IssueId, anyhow::Result<String, anyhow::Error>)>,
) -> (Vec<(IssueId, String)>, FailedIssues) {
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

fn group_redmine_notes(notes: Vec<(IssueId, anyhow::Result<()>)>) -> (Vec<IssueId>, FailedIssues) {
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

    // ????????????ID?????????
    let record_id: u16 = map
        .get("record_issue_number")
        .ok_or_else(|| MyError::ArgIsNotFound("record_issue_number".to_string()))?
        .to_owned()
        .try_into()?;
    let record = module
        .record_usecase()
        .find(IssueId::new(record_id))
        .await
        .with_context(|| format!("?????????????????????????????????????????????????????????: #{:?}", record_id))?;

    // Issue???????????????????????????????????????
    let excluded_ideas: Option<String> = map
        .get("idea_issue_number_exceptions")
        .and_then(|i| i.to_owned().try_into().ok());
    let excluded_ideas = match excluded_ideas {
        Some(v) => refine_all_agendas(v, &record.relations, &module).await?,
        None => vec![],
    }
    .iter()
    .map(|dto| dto.id)
    .collect_vec();

    // Issue????????????????????????????????????
    let ideas = record
        .relations
        .iter()
        .filter(|id| !excluded_ideas.contains(id))
        .map(|v| v.0.to_string())
        .join(" ");
    let ideas = refine_all_agendas(ideas, &record.relations, &module).await?;
    let ideas = ideas
        .iter()
        .filter(|dto| dto.status == AgendaStatus::Approved)
        .collect_vec();

    info!("Create GitHub issues");
    // GitHub???Issue?????????
    let gh_issues = ideas
        .iter()
        .map(|idea| {
            let title = format!("Redmine Idea {}", idea.id.formatted());
            let content = format!(
                "{}\n[{}]({})????????????????????????????????????",
                idea.url(),
                record.discussion_title(),
                record.url()
            );

            (
                idea.id,
                GHIssue::new(
                    title,
                    content,
                    vec!["Tracked: Redmine", "Status/Idea: Accepted???"]
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
        .title("GitHub???Issue????????????????????????????????????????????????")
        .to_owned();

    info!("Add Redmine notes");
    // Redmine???GitHub???Issue???URL?????????
    let redmine_notes = gh_issues
        .iter()
        .map(|(id, gh_issue_url)| {
            (
                *id,
                Note::new(
                    vec![
                        "GitHub???Issue??????????????????????????????URL???????????????????????????",
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
        .title("Redmine????????????????????????????????????????????????")
        .to_owned();

    let success_embed = CreateEmbed::default()
        .custom_default(&record.id)
        .title("GitHub???????????????Redmine?????????????????????????????????????????????????????????????????????")
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

    // ?????????????????????
    interaction
        .send(&ctx.http, InteractionResponse::Embeds(results))
        .await
        .map(|_| ())
}
