use crate::{
    module::{Module, ModuleExt},
    shared::ext::IdExt,
};
use crate_domain::{id::IssueId, status::AgendaStatus};
use crate_usecase::model::AgendaDto;

use anyhow::ensure;
use futures::stream::{self, StreamExt};
use itertools::Itertools;
use log::debug;

fn parse_string_as_issue_ids(idea_args: String) -> anyhow::Result<Vec<IssueId>> {
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

    Ok(ideas)
}

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

fn refine_all_fetched_ideas(
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
pub async fn refine_all_approved_agendas(
    idea_args: String,
    relations: &[IssueId],
    module: &Module,
) -> anyhow::Result<Vec<AgendaDto>> {
    let ideas = parse_string_as_issue_ids(idea_args)?;
    let related = refine_all_related_ideas(ideas, relations)?;
    let fetch_agenda_results: Vec<_> = stream::iter(related)
        .then(|id| fetch_agendas(module, id))
        .collect()
        .await;
    let fetched = refine_all_fetched_ideas(fetch_agenda_results)?;

    refine_all_approved_ideas(fetched)
}
