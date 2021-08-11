use chrono::Utc;
use serenity::{builder, utils::Color};

use crate::domains::redmine;

pub fn default_embed(
    embed: &mut builder::CreateEmbed,
    record_id: u16,
) -> &mut builder::CreateEmbed {
    embed
        .timestamp(Utc::now().to_rfc3339())
        .footer(|footer| footer.text(format!("アイデア会議: #{}", record_id)))
}

pub fn default_success_embed(
    embed: &mut builder::CreateEmbed,
    record_id: u16,
) -> &mut builder::CreateEmbed {
    default_embed(embed, record_id).color(Color::from_rgb(87, 199, 255))
}

pub fn default_failure_embed(
    embed: &mut builder::CreateEmbed,
    record_id: u16,
) -> &mut builder::CreateEmbed {
    default_embed(embed, record_id).color(Color::from_rgb(245, 93, 93))
}

pub fn next_agenda_embed(
    embed: &mut builder::CreateEmbed,
    record_id: u16,
    next_agenda_id: Option<u16>,
) -> &mut builder::CreateEmbed {
    if let Some(id) = next_agenda_id {
        default_success_embed(embed, record_id)
            .title(format!("次の議題は#{}です", id))
            .field(
                "議題チケット",
                format!("{}{}", redmine::REDMINE_ISSUE_URL, id),
                false,
            )
    } else {
        default_failure_embed(embed, record_id)
            .title("次の議題はありません")
            .description("Redmine上で提起されていた議題は全て処理されました。")
    }
}
