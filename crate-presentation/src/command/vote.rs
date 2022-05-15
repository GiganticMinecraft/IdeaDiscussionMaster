use super::super::{global, module::ModuleExt, utils::discord_embeds};
use crate_domain::{error::MyError, status::AgendaStatus};
use crate_shared::{
    command::{
        application_interaction::{ApplicationInteractions, SlashCommand},
        builder::{SlashCommandBuilder, SlashCommandOptionBuilder},
        CommandExt, CommandResult, ExecutorArgs, InteractionResponse, SlashCommandChoice,
    },
    CreateEmbedExt, IdExt,
};

use anyhow::{bail, ensure};
use serenity::{
    builder::CreateEmbed,
    http::Http,
    model::{
        channel::{Message, ReactionType},
        interactions::application_command::{
            ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
        },
    },
};
use std::{boxed::Box, collections::HashMap, str::FromStr};
use strum::IntoEnumIterator;
use tokio::time::{self, Duration};

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

pub async fn start((_map, ctx, interaction): ExecutorArgs) -> CommandResult {
    let record_id = global::record_id::get().ok_or(MyError::DiscussionHasNotStartedYet)?;

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

    let message = interaction
        .send(&ctx.http, InteractionResponse::Embed(embed))
        .await?;
    // リアクション
    for status in AgendaStatus::closed().into_iter() {
        let _ = message.react(&ctx.http, status).await;
    }

    // vote_message_idを格納
    global::agendas::update_votes_message_id(current_agenda.id, Some(message.id));

    let vc_id = global::voice_chat_channel_id::get().ok_or(MyError::IsNotJoinedInVC)?;
    // 投票Embedのリアクションを取得し、VC参加者の過半数を超えていれば/vote endを叩く
    loop {
        // end_votesコマンド等で議題が次に行っている場合処理を終了させないと永遠にループする
        if global::agendas::find_current().map(|agenda| agenda.id) != Some(current_agenda.id) {
            break;
        }

        let vc_members_count =
            crate_shared::get_voice_states(&ctx.cache, &message.guild_id.unwrap())
                .await?
                .iter()
                .filter(|(_, state)| state.channel_id.unwrap_or_default() == vc_id)
                .count();
        if let Some(status) = get_votes_result(&message, &ctx.http, vc_members_count).await {
            // end_votesコマンドを強制的に叩く
            let map = HashMap::from([(
                "status".to_string(),
                ApplicationInteractions::SlashCommand(SlashCommand::Option(Box::new(
                    ApplicationCommandInteractionDataOptionValue::String(status.to_string()),
                ))),
            )]);
            let _ = end((map, ctx, interaction)).await;

            break;
        };

        time::sleep(Duration::from_secs(2)).await;
    }

    Ok(())
}

async fn get_votes_result(
    message: &Message,
    http: impl AsRef<Http>,
    vc_members_count: usize,
) -> Option<AgendaStatus> {
    let half_of_vc_members_count = vc_members_count / 2;

    for status in AgendaStatus::iter() {
        if let Ok(users_count) = message
            .reaction_users(&http, status, Some(100), None)
            .await
            .map(|vector| vector.len())
        {
            if users_count - 1 > half_of_vc_members_count {
                return Some(status);
            }
        }
    }

    None
}

pub async fn end((map, ctx, interaction): ExecutorArgs) -> CommandResult {
    let module = global::module::get();

    // ステータスなど各種必要な変数を取得
    let status: String = map
        .get("status")
        .ok_or_else(|| MyError::ArgIsNotFound("status".to_string()))?
        .to_owned()
        .try_into()?;
    let status = AgendaStatus::from_str(&status).unwrap();
    let record_id = global::record_id::get().ok_or(MyError::DiscussionHasNotStartedYet)?;
    let current_agenda = match global::agendas::find_current() {
        Some(agenda) => agenda,
        None => {
            bail!("現在進行中の議題はありません。")
        }
    };

    // 投票メッセージの記録をリセット
    if current_agenda.votes_message_id.is_some() {
        global::agendas::update_votes_message_id(current_agenda.id, None);
    }

    // 投票結果のEmbedを作成
    let mut vote_result_embed = CreateEmbed::default();
    let vote_result_embed = {
        vote_result_embed.custom_default(&record_id);

        match status {
            AgendaStatus::Approved => vote_result_embed.success_color(),
            AgendaStatus::Declined => vote_result_embed.failure_color(),
            _ => &mut vote_result_embed,
        }
        .title(format!(
            "採決終了: {}は{}されました",
            current_agenda.id.formatted(),
            status.ja()
        ))
        .to_owned()
    };

    // ステータスに応じてRedmineとグローバル変数を更新
    match status {
        AgendaStatus::Approved => {
            let _ = module.agenda_usecase().approve(current_agenda.id).await;
            global::agendas::approve(current_agenda.id);
        }
        AgendaStatus::Declined => {
            let _ = module.agenda_usecase().decline(current_agenda.id).await;
            global::agendas::decline(current_agenda.id);
        }
        _ => {}
    };

    // 次の議題を選択
    let next_agenda_id = global::agendas::find_next();
    // 次の議題があれば、グローバル変数の中の議題のステータスを進行中に変更
    if let Some(id) = next_agenda_id {
        global::agendas::in_progress(id);
    }
    // 次の議題の存否に応じてEmbedを作成
    let mut agenda_embed = CreateEmbed::default();
    let agenda_embed = match next_agenda_id {
        Some(id) => {
            let next_agenda = module.agenda_usecase().find_new(id).await?;

            println!("Next Agenda: {}", next_agenda_id.unwrap().formatted());

            discord_embeds::next_agenda_embed(&mut agenda_embed, &record_id, &next_agenda)
        }
        None => discord_embeds::no_next_agenda(&mut agenda_embed, &record_id),
    }
    .to_owned();

    interaction
        .send(
            &ctx.http,
            InteractionResponse::Embeds(vec![vote_result_embed, agenda_embed]),
        )
        .await
        .map(|_| ())
}
