use super::super::{global, module::ModuleExt, utils::discord_embeds};
use crate_domain::{error::MyError, id::IssueId};
use crate_shared::{
    command::{
        builder::{SlashCommandBuilder, SlashCommandOptionBuilder},
        CommandExt, CommandResult, ExecutorArgs, InteractionResponse,
    },
    ext::{CreateEmbedExt, IdExt},
};

use serenity::{
    builder::CreateEmbed, model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("agenda", "議題の操作を行います。")
        .add_option(
            SlashCommandOptionBuilder::new(
                "add",
                "議題を追加します。",
                ApplicationCommandOptionType::SubCommand,
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "idea_issue_number",
                    "追加する議題のチケット番号",
                    ApplicationCommandOptionType::Integer,
                )
                .min_int(1)
                .required(true),
            ),
        )
        .add_option(SlashCommandOptionBuilder::new(
            "list",
            "議題の一覧を表示します。",
            ApplicationCommandOptionType::SubCommand,
        ))
        .into()
}

pub async fn add((map, ctx, interaction): ExecutorArgs) -> CommandResult {
    let module = global::module::get();

    // 新規議題を取得し、グローバル変数に格納
    // 新規議題が存在しなければ、終了
    let new_agenda_id: u16 = map
        .get("idea_issue_number")
        .ok_or_else(|| MyError::ArgIsNotFound("idea_issue_number".to_string()))?
        .to_owned()
        .try_into()?;
    let new_agenda_id = IssueId::new(new_agenda_id);
    let new_agenda = module.agenda_usecase().find_new(new_agenda_id).await?;
    global::agendas::add(new_agenda.clone().into());

    // 議事録チケットと関連付ける
    let record_id = global::record_id::get().ok_or(MyError::DiscussionHasNotStartedYet)?;
    println!("{} {}", record_id.0, new_agenda_id.0);
    module
        .record_usecase()
        .add_relation(record_id, new_agenda_id)
        .await?;

    let add_agenda_embed = CreateEmbed::default()
        .custom_default(&record_id)
        .title("議題を追加しました")
        .description(format!("追加した議題: {}", new_agenda_id.formatted()))
        .success_color()
        .to_owned();

    println!("Agenda added: {}", new_agenda_id.formatted());

    // 現在進行中の議題があれば何もせず、なければ議題として提示
    let response = match global::agendas::find_current() {
        Some(_) => InteractionResponse::Embed(add_agenda_embed),
        None => {
            assert!(global::agendas::find_next().is_some());

            global::agendas::in_progress(new_agenda_id);

            let mut agenda_embed = CreateEmbed::default();
            let agenda_embed =
                discord_embeds::next_agenda_embed(&mut agenda_embed, &record_id, &new_agenda)
                    .to_owned();

            println!("Next Agenda: {}", new_agenda_id.formatted());

            InteractionResponse::Embeds(vec![add_agenda_embed, agenda_embed])
        }
    };

    interaction.send(&ctx.http, response).await.map(|_| ())
}

pub async fn list((_map, ctx, interaction): ExecutorArgs) -> CommandResult {
    let agendas = global::agendas::grouped_list();
    let record_id = global::record_id::get().ok_or(MyError::DiscussionHasNotStartedYet)?;

    let mut result_embed = CreateEmbed::default();
    let result_embed = discord_embeds::agendas_result(&mut result_embed, &record_id, &agendas)
        .title("現在の議題状況")
        .to_owned();

    interaction
        .send(&ctx.http, InteractionResponse::Embed(result_embed))
        .await
        .map(|_| ())
}
