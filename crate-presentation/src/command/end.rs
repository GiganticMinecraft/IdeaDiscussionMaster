use super::super::{global, module::ModuleExt, utils::discord_embeds};
use crate_domain::{id::IssueId, redmine::Note};
use crate_shared::{
    command::{builder::SlashCommandBuilder, CommandResult, ExecutorArgs, InteractionResponse},
    IdExt,
};

use itertools::Itertools;
use serenity::builder::CreateEmbed;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("end", "アイデア会議を終了します。")
}

pub async fn executor((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    let record_id = global::record_id::get().unwrap_or_else(|| IssueId::new(1));
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

    println!("Discussion finished: {}", record_id.formatted());
    println!("Result:\n {}", result_strings);

    // グローバル変数をすべてリセット
    global::voice_chat_channel_id::clear();
    global::agendas::clear();
    global::record_id::clear();

    Ok(InteractionResponse::Embed(result_embed))
}
