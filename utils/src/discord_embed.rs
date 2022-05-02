use super::REDMINE_URL;
use chrono::Utc;
use domain::{id::IssueId, redmine::Agenda, status::AgendaStatus};
use regex::Regex;
use serenity::{builder, utils::Color};

pub trait CreateEmbedExt {
    fn timestamp(&mut self) -> &mut Self;
    fn with_record_id(&mut self, record_id: IssueId) -> &mut Self;
    fn simple_color(&mut self) -> &mut Self;
    fn success_color(&mut self) -> &mut Self;
    fn failure_color(&mut self) -> &mut Self;
}

impl CreateEmbedExt for builder::CreateEmbed {
    fn timestamp(&mut self) -> &mut Self {
        self.timestamp(Utc::now().to_rfc3339())
    }

    fn with_record_id(&mut self, record_id: IssueId) -> &mut Self {
        self.footer(|footer| footer.text(format!("アイデア会議: #{}", record_id.0)))
    }

    fn simple_color(&mut self) -> &mut Self {
        self.color(Color::from_rgb(179, 159, 159))
    }

    fn success_color(&mut self) -> &mut Self {
        self.color(Color::from_rgb(50, 173, 240))
    }

    fn failure_color(&mut self) -> &mut Self {
        self.color(Color::from_rgb(245, 93, 93))
    }
}

pub fn next_agenda_embed(
    embed: &mut builder::CreateEmbed,
    record_id: IssueId,
    next_agenda: Option<Agenda>,
) -> &mut builder::CreateEmbed {
    if let Some(issue) = next_agenda {
        let reg = Regex::new(r"^\[.*\]\s").unwrap();
        let subject = reg.replace(&issue.title, "");

        embed
            .simple_color()
            .with_record_id(record_id)
            .title(format!("次の議題は#{}です", issue.id.0))
            .field(
                "議題チケット",
                format!("{}/issues/{}", REDMINE_URL, issue.id.0),
                false,
            )
            .field("タイトル", subject, false)
            .field("説明", issue.description, false)
    } else {
        embed
            .failure_color()
            .with_record_id(record_id)
            .title("次の議題はありません")
            .description("Redmine上で提起されていた議題は全て処理されました。")
    }
}

pub fn no_current_agenda_embed(
    embed: &mut builder::CreateEmbed,
    record_id: IssueId,
) -> &mut builder::CreateEmbed {
    embed
        .failure_color()
        .with_record_id(record_id)
        .title("現在進行中の議題はありません")
}

pub fn votes_result_embed(
    embed: &mut builder::CreateEmbed,
    record_id: IssueId,
    current_agenda_id: IssueId,
    status: AgendaStatus,
) -> &mut builder::CreateEmbed {
    match status {
        AgendaStatus::Approved => embed.success_color().with_record_id(record_id),
        AgendaStatus::Declined => embed.failure_color().with_record_id(record_id),
        _ => embed,
    }
    .title(format!(
        "採決終了: #{}は{}されました",
        current_agenda_id.0,
        status.ja()
    ))
}
