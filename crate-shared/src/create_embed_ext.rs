use super::REDMINE_URL;
use crate_domain::id::IssueId;

use chrono::Utc;
use serenity::{builder, utils::Color};

pub trait CreateEmbedExt {
    fn custom_default(&mut self, record_id: &IssueId) -> &mut Self;
    fn current_timestamp(&mut self) -> &mut Self;
    fn with_record_id(&mut self, record_id: &IssueId) -> &mut Self;
    fn simple_color(&mut self) -> &mut Self;
    fn success_color(&mut self) -> &mut Self;
    fn failure_color(&mut self) -> &mut Self;
    fn record_url_field(&mut self, record_id: &IssueId) -> &mut Self;
}

impl CreateEmbedExt for builder::CreateEmbed {
    fn custom_default(&mut self, record_id: &IssueId) -> &mut Self {
        self.current_timestamp().with_record_id(record_id)
    }

    fn current_timestamp(&mut self) -> &mut Self {
        self.timestamp(Utc::now().to_rfc3339())
    }

    fn with_record_id(&mut self, record_id: &IssueId) -> &mut Self {
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

    fn record_url_field(&mut self, record_id: &IssueId) -> &mut Self {
        self.field(
            "議事録チケット",
            format!("{}/issues/{}", REDMINE_URL, record_id.0),
            false,
        )
    }
}
