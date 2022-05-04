use crate_domain::id::IssueId;

use chrono::Utc;
use serenity::{builder, utils::Color};

pub trait CreateEmbedExt {
    fn timestamp(&mut self) -> &mut Self;
    fn with_record_id(&mut self, record_id: &IssueId) -> &mut Self;
    fn simple_color(&mut self) -> &mut Self;
    fn success_color(&mut self) -> &mut Self;
    fn failure_color(&mut self) -> &mut Self;
}

impl CreateEmbedExt for builder::CreateEmbed {
    fn timestamp(&mut self) -> &mut Self {
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
}
