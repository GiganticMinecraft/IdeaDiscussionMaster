use super::super::global;
use crate_shared::{
    command::{
        builder::{SlashCommandBuilder, SlashCommandOptionBuilder},
        CommandResult, ExecutorArgs, InteractionResponse, SlashCommandChoice,
    },
    CreateEmbedExt, IdExt,
};

use anyhow::{bail, ensure};
use serenity::{
    builder::CreateEmbed, model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("vote", "投票を行います。")
        .add_option(SlashCommandOptionBuilder::new(
            "start",
            "投票を開始します。",
            ApplicationCommandOptionType::SubCommand,
        ))
        .add_option(
            SlashCommandOptionBuilder::new(
                "end",
                "指定したステータスで投票を終了します。",
                ApplicationCommandOptionType::SubCommand,
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "status",
                    "変更後のステータス",
                    ApplicationCommandOptionType::Integer,
                )
                .add_choice(("Approved", SlashCommandChoice::Int(1)))
                .add_choice(("Declined", SlashCommandChoice::Int(2)))
                .required(true),
            ),
        )
        .into()
}

pub async fn start((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    let record_id = global::record_id::get().unwrap();

    let current_agenda = match global::agendas::find_current() {
        Some(agenda) => agenda,
        None => {
            bail!("現在進行中の議題はありません。")
        }
    };
    ensure!(
        current_agenda.votes_message_id.is_none(),
        "すでに採決を開始しています。"
    );

    let embed_description = vec![
        "提起されている議題についての採決を行います。",
        "以下のリアクションで投票を行ってください。過半数を超え次第、次の議題へと移ります。",
        ":o:: 承認",
        ":x:: 却下",
        "",
        "※リアクションがすべて表示されてからリアクションを行わないと、投票が無効になる場合があります。",
    ]
    .join("\n");
    let embed = CreateEmbed::default()
        .custom_default(&record_id)
        .title(format!("採決: {}", record_id.formatted()))
        .description(embed_description)
        .to_owned();
    // TODO: react for embed
    // TODO: update vote message id

    Ok(InteractionResponse::Embed(embed))
}

pub async fn end((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    Ok(InteractionResponse::Message("vote end".to_string()))
}
