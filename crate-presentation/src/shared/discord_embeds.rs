use crate::{
    global::model::Agenda,
    shared::ext::{CreateEmbedExt, IdExt},
};
use crate_domain::{id::IssueId, status::AgendaStatus};
use crate_usecase::model::{AgendaDto, DtoExt};

use itertools::Itertools;
use regex::Regex;
use serenity::builder;

pub fn next_agenda_embed<'a>(
    embed: &'a mut builder::CreateEmbed,
    record_id: &IssueId,
    next_agenda: &AgendaDto,
) -> &'a mut builder::CreateEmbed {
    let reg = Regex::new(r"^\[.*\]\s").unwrap();
    let subject = reg.replace(&next_agenda.title, "");

    embed
        .custom_default(record_id)
        .simple_color()
        .title(format!("次の議題は{}です", next_agenda.id.formatted()))
        .custom_field("議題チケット", next_agenda.url(), false)
        .custom_field("タイトル", subject, false)
        .custom_field("説明", next_agenda.description.clone(), false)
}

pub fn no_next_agenda<'a>(
    embed: &'a mut builder::CreateEmbed,
    record_id: &IssueId,
) -> &'a mut builder::CreateEmbed {
    embed
        .custom_default(record_id)
        .failure_color()
        .title("次の議題はありません")
        .description("Redmine上で提起されていた議題は全て処理されました。")
}

pub fn agendas_result<'a>(
    embed: &'a mut builder::CreateEmbed,
    record_id: &'a IssueId,
    agenda_list: &'a [(AgendaStatus, Vec<Agenda>)],
) -> &'a mut builder::CreateEmbed {
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
                    .map(|agenda| agenda.id.formatted())
                    .join(", "),
                // フィールドをインラインにするかどうか
                false,
            )
        })
        .collect_vec();

    embed
        .custom_default(record_id)
        .record_url_field(record_id)
        .custom_fields(agenda_fields)
}
