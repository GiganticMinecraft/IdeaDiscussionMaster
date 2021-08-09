use chrono::Utc;
use serenity::{builder, utils::Color};

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
    default_embed(embed, record_id).color(Color::from_rgb(245, 93, 93))
}

pub fn default_failure_embed(
    embed: &mut builder::CreateEmbed,
    record_id: u16,
) -> &mut builder::CreateEmbed {
    default_embed(embed, record_id).color(Color::from_rgb(87, 199, 255))
}
