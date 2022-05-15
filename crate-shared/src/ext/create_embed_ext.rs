use super::{super::REDMINE_URL, IdExt};
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
    fn custom_field<T, U>(&mut self, name: T, value: U, inline: bool) -> &mut Self
    where
        T: ToString,
        U: ToString;
    fn custom_fields<T, U, It>(&mut self, fields: It) -> &mut Self
    where
        T: ToString,
        U: ToString,
        It: IntoIterator<Item = (T, U, bool)>;
    fn is_empty(&self) -> bool;
}

impl CreateEmbedExt for builder::CreateEmbed {
    fn custom_default(&mut self, record_id: &IssueId) -> &mut Self {
        self.current_timestamp().with_record_id(record_id)
    }

    fn current_timestamp(&mut self) -> &mut Self {
        self.timestamp(Utc::now().to_rfc3339())
    }

    fn with_record_id(&mut self, record_id: &IssueId) -> &mut Self {
        self.footer(|footer| footer.text(format!("アイデア会議: {}", record_id.formatted())))
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
        self.custom_field(
            "議事録チケット",
            format!("{}/issues/{}", REDMINE_URL, record_id.0),
            false,
        )
    }

    fn custom_field<T, U>(&mut self, name: T, value: U, inline: bool) -> &mut Self
    where
        T: ToString,
        U: ToString,
    {
        let name = name.to_string();
        let name = if name.is_empty() {
            "-".to_string()
        } else {
            name
        };

        let value = value.to_string();
        let value = if value.is_empty() {
            "-".to_string()
        } else {
            value
        };

        self.field(name, value, inline)
    }
    fn custom_fields<T, U, It>(&mut self, fields: It) -> &mut Self
    where
        T: ToString,
        U: ToString,
        It: IntoIterator<Item = (T, U, bool)>,
    {
        fields.into_iter().for_each(|(name, value, inline)| {
            self.custom_field(name, value, inline);
        });

        self
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
