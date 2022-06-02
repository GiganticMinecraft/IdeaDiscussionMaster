use crate::{
    global,
    module::ModuleExt,
    shared::{
        builder::SlashCommandBuilder,
        command::{CommandResult, ExecutorArgs, InteractionResponse},
        discord_embeds, CommandExt,
    },
};
use crate_domain::{error::MyError, redmine::Note};
use crate_shared::ext::IdExt;

use itertools::Itertools;
use log::info;
use serenity::builder::CreateEmbed;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("end", "アイデア会議を終了します。")
}

pub async fn executor((_map, ctx, interaction): ExecutorArgs) -> CommandResult {
    let record_id = global::record_id::get().ok_or(MyError::DiscussionHasNotStartedYet)?;
    let result = global::agendas::grouped_list();

    // Embedを作る
    let mut result_embed = CreateEmbed::default();
    let result_embed = discord_embeds::agendas_result(&mut result_embed, &record_id, &result)
        .title("会議を終了しました")
        .to_owned();

    // 議事録に議論の結果を記載し、チケットを終了する
    // 議論の結果を見やすくStringに
    let result_strings = result
        .iter()
        .map(|(status, agendas)| {
            format!(
                "[{}]\n{}\n",
                status.ja(),
                agendas.iter().map(|agenda| agenda.id.formatted()).join(" ")
            )
        })
        .join("\n");

    let module = global::module::get();
    let _ = module
        .record_usecase()
        .add_note(record_id, Note::from_string_content(result_strings.clone()))
        .await?;
    let _ = module.record_usecase().close(record_id).await?;

    info!("Discussion finished: {}", record_id.formatted());
    info!("Result:\n {}", result_strings);

    // グローバル変数をすべてリセット
    global::voice_chat_channel_id::clear();
    global::agendas::clear();
    global::record_id::clear();

    interaction
        .send(&ctx.http, InteractionResponse::Embed(result_embed))
        .await
        .map(|_| ())
}
