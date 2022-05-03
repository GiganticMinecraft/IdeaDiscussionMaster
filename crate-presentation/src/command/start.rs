use super::super::{global, module::ModuleExt};
use crate_domain::id::IssueId;
use crate_utils::{
    self,
    command::{
        builder::{SlashCommandBuilder, SlashCommandOptionBuilder},
        CommandResult, ExecutorArgs, InteractionResponse,
    },
};

use serenity::model::interactions::application_command::ApplicationCommandOptionType;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("start", "アイデア会議を開始します。")
        .add_option(
            SlashCommandOptionBuilder::new(
                "discussion_issue_number",
                "議事録のチケット番号",
                ApplicationCommandOptionType::Integer,
            )
            .min_int(1)
            .max_int(u16::MAX.into())
            .required(true),
        )
        .into()
}

pub async fn executor((map, ctx, interaction): ExecutorArgs) -> CommandResult {
    let vc_id = crate_utils::find_vc_by_user_id(
        &ctx.cache,
        &interaction.guild_id.unwrap(),
        &interaction.user.id,
    )
    .await?;
    global::voice_chat_channel_id::update(vc_id);

    let record_id: u16 = map
        .get("discussion_issue_number")
        .unwrap()
        .to_owned()
        .try_into()?;
    let record_id = IssueId::new(record_id);
    let module = global::module::get();
    let record = module.record_usecase().find_new(record_id).await?;

    Ok(InteractionResponse::Message(format!(
        "会議を開始しました。\n {:?}",
        record
    )))
}
