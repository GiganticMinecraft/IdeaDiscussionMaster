use crate::{
    serenity::builder,
    shared::ext::{CreateEmbedExt, UseFormattedId, UseStatusJa},
};
use c_domain::status::AgendaStatus;
use c_usecase::model::{AgendaDto, RecordDto};

use itertools::Itertools;
use regex::Regex;

pub fn next_agenda_embed<'a>(
    embed: &'a mut builder::CreateEmbed,
    record: &RecordDto,
    next_agenda: &AgendaDto,
) -> &'a mut builder::CreateEmbed {
    let reg = Regex::new(r"^\[.*]\s").unwrap();
    let subject = reg.replace(&next_agenda.title, "");

    embed
        .custom_default(record)
        .simple_color()
        .title(format!(
            "次の議題は{}です",
            next_agenda.id.as_formatted_id()
        ))
        .custom_field("議題チケット", next_agenda.url(), false)
        .custom_field("タイトル", subject, false)
        .custom_field("説明", next_agenda.description.clone(), false)
}

pub fn no_next_agenda<'a>(
    embed: &'a mut builder::CreateEmbed,
    record: &RecordDto,
) -> &'a mut builder::CreateEmbed {
    embed
        .custom_default(record)
        .failure_color()
        .title("次の議題はありません")
        .description("Redmine上で提起されていた議題は全て処理されました。")
}

pub fn agendas_result(
    embed: &mut builder::CreateEmbed,
    record: RecordDto,
    agenda_list: Vec<(AgendaStatus, Vec<AgendaDto>)>,
) -> &mut builder::CreateEmbed {
    let agenda_fields = agenda_list
        .iter()
        .map(|(status, agendas)| {
            // tupleにしておくことで、そのままCreateEmbed#fieldsに渡せる
            (
                // フィールド名
                status.ja(),
                // フィールドの内容
                agendas
                    .iter()
                    .map(|agenda| agenda.formatted_id())
                    .join(", "),
                // フィールドをインラインにするかどうか
                false,
            )
        })
        .collect_vec();

    embed
        .custom_default(&record)
        .record_url_field(&record)
        .custom_fields(agenda_fields)
}
