use crate::shared::{
    ext::{CreateEmbedExt, CutString, UseFormattedId, UseStatusJa},
    VoteChoice, VoteChoiceWithId,
};
use c_domain::redmine::model::{
    id::{AgendaId, RecordId},
    status::AgendaStatus,
};
use c_usecase::redmine::model::{AgendaDto, RecordDto};

use itertools::Itertools;
use poise::serenity_prelude::CreateEmbed;
use regex::Regex;

pub fn next_agenda_embed<'a>(
    embed: &'a mut CreateEmbed,
    record_id: &RecordId,
    next_agenda: &AgendaDto,
) -> &'a mut CreateEmbed {
    let reg = Regex::new(r"^\[.*]\s").unwrap();
    let subject = reg.replace(&next_agenda.title, "").to_string();
    let subject = subject.cut_at(100);
    let description = next_agenda.description.cut_at(1500);

    embed
        .custom_default(record_id)
        .simple_color()
        .title(format!(
            "次の議題は{}です",
            AgendaId::new(next_agenda.id).formatted()
        ))
        .custom_field("議題チケット", next_agenda.url(), false)
        .custom_field("タイトル", subject, false)
        .custom_field("説明", description, false)
}

pub fn no_next_agenda<'a>(embed: &'a mut CreateEmbed, record_id: &RecordId) -> &'a mut CreateEmbed {
    embed
        .custom_default(record_id)
        .failure_color()
        .title("次の議題はありません")
        .description("Redmine上で提起されていた議題は全て処理されました。")
}

pub fn agendas_result(
    embed: &mut CreateEmbed,
    record: RecordDto,
    agenda_list: Vec<(AgendaStatus, Vec<AgendaDto>)>,
) -> &mut CreateEmbed {
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
                    .map(|agenda| AgendaId::new(agenda.id).formatted())
                    .join(", "),
                // フィールドをインラインにするかどうか
                false,
            )
        })
        .collect_vec();

    embed
        .custom_default(&RecordId::new(record.id))
        .record_url_field(&record)
        .custom_fields(agenda_fields)
}

pub fn vote_progress(embed: &mut CreateEmbed, votes: Vec<VoteChoiceWithId>) -> &mut CreateEmbed {
    let votes = votes
        .into_iter()
        .counts()
        .into_iter()
        .sorted_by_key(|value| value.1)
        .rev()
        .map(|((id, choice), count)| format!("{} {}: {}", id, choice.status.ja(), count))
        .join("\n");

    embed
        .current_timestamp()
        .title("投票状況")
        .description(if votes.is_empty() {
            "票はありません".to_string()
        } else {
            votes
        })
}

pub fn vote_result<'a>(
    embed: &'a mut CreateEmbed,
    record_id: &RecordId,
    current_agenda_id: &AgendaId,
    choice: &VoteChoice,
) -> &'a mut CreateEmbed {
    match choice.status {
        AgendaStatus::Approved => embed.success_color(),
        AgendaStatus::Declined => embed.failure_color(),
        _ => embed,
    };

    embed
        .custom_default(record_id)
        .title(format!(
            "投票終了: {}は{}されました",
            current_agenda_id.formatted(),
            choice.status.ja()
        ))
        .description(format!("{}", choice))
}
