use c_domain::redmine::model::id::AgendaId;

use anyhow::ensure;
use itertools::Itertools;

pub fn parse_string_as_agenda_ids(idea_arg: String) -> anyhow::Result<Vec<AgendaId>> {
    let ideas = idea_arg
        .split(' ')
        .filter_map(|str| str.parse().ok())
        .map(AgendaId::new)
        .collect_vec();
    ensure!(
        !ideas.is_empty(),
        "指定された文字列を議題のリストとして認識できません: {}",
        idea_arg
    );

    Ok(ideas)
}
