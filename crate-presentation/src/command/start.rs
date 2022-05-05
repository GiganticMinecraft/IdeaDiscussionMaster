use super::super::{global, module::ModuleExt, utils::discord_embeds};
use crate_domain::id::IssueId;
use crate_shared::{
    self,
    command::{
        builder::{SlashCommandBuilder, SlashCommandOptionBuilder},
        CommandResult, ExecutorArgs, InteractionResponse,
    },
    CreateEmbedExt, REDMINE_URL,
};

use futures::stream::{self, StreamExt};
use itertools::Itertools;
use serenity::{
    builder::CreateEmbed, model::interactions::application_command::ApplicationCommandOptionType,
};

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
    let module = global::module::get();

    // VCへの参加状況を取得
    // 参加していればグローバル変数にそのVCのChannelIdを格納
    // 参加していなければ終了
    // let vc_id = crate_shared::find_vc_by_user_id(
    //     &ctx.cache,
    //     &interaction.guild_id.unwrap(),
    //     &interaction.user.id,
    // )
    // .await?;
    // global::voice_chat_channel_id::update(vc_id);

    // 議事録を取得
    // 存在すれば、グローバル変数に格納
    // 存在しなければ、終了
    let record_id: u16 = map
        .get("discussion_issue_number")
        .unwrap()
        .to_owned()
        .try_into()?;
    let record_id = IssueId::new(record_id);
    let record = module.record_usecase().find_new(record_id).await?;
    global::record_id::update(record_id);

    // 議題を取得
    let agendas: Vec<_> = stream::iter(&record.relations)
        .then(|id| module.agenda_usecase().find_new(*id))
        .filter_map(|res| async { res.ok() })
        .collect()
        .await;
    // グローバル変数の議題をリセットして、取得した議題を追加
    global::agendas::clear();
    agendas
        .clone()
        .into_iter()
        .map(|dto| dto.into())
        .for_each(|agenda| {
            global::agendas::add(agenda);
        });

    let beginning_embed = CreateEmbed::default()
        .custom_default(&record_id)
        .title("会議を開始しました")
        .field(
            "議事録チケット",
            format!("{}/issues/{}", REDMINE_URL, record_id.0),
            false,
        )
        .to_owned();

    println!("Discussion started: #{}", record_id.0);
    println!(
        "Agendas({}): {:?}",
        agendas.len(),
        agendas
            .iter()
            .map(|agenda| format!("#{}", agenda.id.0))
            .join(", ")
    );

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

            println!("Next Agenda: #{}", next_agenda_id.unwrap().0);

            discord_embeds::next_agenda_embed(&mut agenda_embed, &record_id, &next_agenda)
        }
        None => discord_embeds::no_next_agenda(&mut agenda_embed, &record_id),
    }
    .to_owned();

    Ok(InteractionResponse::Embeds(vec![
        beginning_embed,
        agenda_embed,
    ]))
}
