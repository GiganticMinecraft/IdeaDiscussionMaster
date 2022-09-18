use crate::commands::{vote::shared::end_votes, CommandResult, Context};
use c_domain::redmine::model::status::AgendaStatus;

use anyhow::anyhow;
use itertools::Itertools;
use poise::futures_util::{future, stream, Stream, StreamExt};

async fn autocomplete_agenda_status_string<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    stream::iter(
        AgendaStatus::closed()
            .into_iter()
            .map(|status| status.to_string()),
    )
    .filter(move |name| future::ready(name.to_lowercase().starts_with(&partial.to_lowercase())))
}

/// 採決を終了します
#[poise::command(slash_command)]
pub async fn end(
    ctx: Context<'_>,
    #[description = "変更後のステータス"]
    #[autocomplete = "autocomplete_agenda_status_string"]
    status: String,
) -> CommandResult {
    let status = AgendaStatus::from_string(&status).ok_or_else(|| {
        anyhow!(
            "議題のステータスは{}のうちいずれか1つのみ指定できます",
            AgendaStatus::closed()
                .iter()
                .map(|status| format!("「{:?}」", status))
                .collect_vec()
                .join("、")
        )
    })?;
    end_votes(&ctx, status).await?;

    Ok(())
}
