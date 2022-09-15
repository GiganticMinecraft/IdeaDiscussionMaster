use crate::{
    commands::{CommandResult, Context},
    shared::CommandError,
};
use c_domain::id::RecordId;

/// 採決を終了します
#[poise::command(slash_command)]
pub async fn end(ctx: Context<'_>) -> CommandResult {
    let _record_id = ctx
        .data()
        .record_id
        .get()
        .map(RecordId::new)
        .ok_or(CommandError::DiscussionHasBeenStarted)?;

    Ok(())
}
