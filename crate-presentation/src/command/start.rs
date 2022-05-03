use crate_utils::command::{
    builder::{SlashCommandBuilder, SlashCommandOptionBuilder},
    force_boxed, CommandArg, CommandResult, InteractionResponse,
};

use serenity::model::interactions::application_command::ApplicationCommandOptionType;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new(
        "start",
        "アイデア会議を開始します。",
        Some(force_boxed(start)),
    )
    .add_option(
        SlashCommandOptionBuilder::new(
            "discussion_issue_number",
            "議事録のチケット番号",
            ApplicationCommandOptionType::Integer,
            None,
        )
        .min_int(1)
        .max_int(u16::MAX.into())
        .required(true),
    )
    .into()
}

async fn start(map: CommandArg) -> CommandResult {
    let n: i64 = map
        .get("discussion_issue_number")
        .unwrap()
        .to_owned()
        .try_into()
        .unwrap();

    Ok(InteractionResponse::Message(format!(
        "会議が始まりました: 指定された議事録チケット: {}",
        n
    )))
}
