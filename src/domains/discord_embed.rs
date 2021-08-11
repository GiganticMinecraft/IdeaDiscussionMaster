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

pub fn default_colored_embed(
    embed: &mut builder::CreateEmbed,
    record_id: u16,
) -> &mut builder::CreateEmbed {
    default_embed(embed, record_id).color(Color::from_rgb(179, 159, 159))
}

pub fn default_success_embed(
    embed: &mut builder::CreateEmbed,
    record_id: u16,
) -> &mut builder::CreateEmbed {
    default_embed(embed, record_id).color(Color::from_rgb(50, 173, 240))
}

pub fn default_failure_embed(
    embed: &mut builder::CreateEmbed,
    record_id: u16,
) -> &mut builder::CreateEmbed {
    default_embed(embed, record_id).color(Color::from_rgb(245, 93, 93))
}

// TODO: タイトルと説明を表示する
pub fn next_agenda_embed(
    embed: &mut builder::CreateEmbed,
    record_id: u16,
    next_redmine_issue: Option<redmine::RedmineIssue>,
) -> &mut builder::CreateEmbed {
    if let Some(issue) = next_redmine_issue {
        default_colored_embed(embed, record_id)
            .title(format!("次の議題は#{}です", issue.id))
            .field(
                "議題チケット",
                format!("{}{}", redmine::REDMINE_ISSUE_URL, issue.id),
                false,
            )
    } else {
        default_failure_embed(embed, record_id)
            .title("次の議題はありません")
            .description("Redmine上で提起されていた議題は全て処理されました。")
    }
}
