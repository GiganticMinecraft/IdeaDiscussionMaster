use super::super::{
    global::{self, model::Agenda},
    module::ModuleExt,
};
use crate_domain::{id::IssueId, redmine::Note, status::AgendaStatus};
use crate_shared::{
    command::{builder::SlashCommandBuilder, CommandResult, ExecutorArgs, InteractionResponse},
    CreateEmbedExt,
};

use itertools::Itertools;
use serenity::builder::CreateEmbed;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("end", "アイデア会議を終了します。")
}

pub async fn executor((_map, _ctx, _interaction): ExecutorArgs) -> CommandResult {
    let record_id = global::record_id::get().unwrap_or_else(|| IssueId::new(1));
    // 議題をすべて取得し、ステータスでソート
    // ここでソートしないと、そのままの順番でグルーピングされるので、同じステータスの別グループができる
    let agendas = global::agendas::list()
        .into_iter()
        .sorted_by_cached_key(|agenda| agenda.status)
        .collect_vec();
    // 議題をステータスでグルーピング
    // https://stackoverflow.com/questions/47885478/how-to-use-itertools-group-by-iterator-method-without-a-for-loop
    let result: Vec<(AgendaStatus, Vec<Agenda>)> = agendas
        .iter()
        .group_by(|agenda| agenda.status)
        .into_iter()
        .map(|(status, group)| (status, group.cloned().collect()))
        .collect();

    // Embedを作る
    let result_fields = result
        .iter()
        .map(|(status, agendas)| {
            // tupleにしておくことで、そのままCreateEmbed#fieldsに渡せる
            (
                // フィールド名
                status.ja(),
                // フィールドの内容
                agendas
                    .iter()
                    .map(|agenda| format!("#{}", agenda.id.0))
                    .join(", "),
                // フィールドをインラインにするかどうか
                false,
            )
        })
        .collect_vec();
    let result_embed = CreateEmbed::default()
        .custom_default(&record_id)
        .title("会議を終了しました")
        .record_url_field(&record_id)
        .fields(result_fields)
        .to_owned();

    // 議事録に議論の結果を記載し、チケットを終了する
    // 議論の結果を見やすくStringに
    let result_strings = result
        .iter()
        .map(|(status, agendas)| {
            format!(
                "[{}]\n{}\n",
                status.ja(),
                agendas
                    .iter()
                    .map(|agenda| format!("#{}", agenda.id.0))
                    .join(" ")
            )
        })
        .join("\n");

    let module = global::module::get();
    let _ = module
        .record_usecase()
        .add_note(record_id, Note::from_string_content(result_strings.clone()))
        .await?;
    let _ = module.record_usecase().close(record_id).await?;

    println!("Discussion finished: #{}", record_id.0);
    println!("Result:\n {}", result_strings);

    // グローバル変数をすべてリセット
    global::voice_chat_channel_id::clear();
    global::agendas::clear();
    global::record_id::clear();

    Ok(InteractionResponse::Embed(result_embed))
}
