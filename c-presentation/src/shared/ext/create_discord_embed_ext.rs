use crate::shared::ext::UseFormattedId;
use c_domain::redmine::model::id::RecordId;
use c_usecase::redmine::model::RecordDto;

use poise::serenity_prelude::{Color, CreateEmbed, CreateEmbedFooter, Timestamp};

pub trait CreateEmbedExt {
    fn custom_default(self, record_id: &RecordId) -> Self;
    fn current_timestamp(self) -> Self;
    fn with_record_id(self, record_id: &RecordId) -> Self;
    fn simple_color(self) -> Self;
    fn success_color(self) -> Self;
    fn failure_color(self) -> Self;
    fn record_url_field(self, record: &RecordDto) -> Self;
    fn custom_field<T, U>(self, name: T, value: U, inline: bool) -> Self
    where
        T: ToString,
        U: ToString;
    fn custom_fields<T, U, It>(self, fields: It) -> Self
    where
        T: ToString,
        U: ToString,
        It: IntoIterator<Item = (T, U, bool)>;
}

impl CreateEmbedExt for CreateEmbed {
    fn custom_default(self, record_id: &RecordId) -> Self {
        self.current_timestamp().with_record_id(record_id)
    }

    fn current_timestamp(self) -> Self {
        self.timestamp(Timestamp::now())
    }

    fn with_record_id(self, record_id: &RecordId) -> Self {
        self.footer(CreateEmbedFooter::new(format!(
            "アイデア会議: {}",
            record_id.formatted()
        )))
    }

    fn simple_color(self) -> Self {
        self.color(Color::from_rgb(179, 159, 159))
    }

    fn success_color(self) -> Self {
        self.color(Color::from_rgb(50, 173, 240))
    }

    fn failure_color(self) -> Self {
        self.color(Color::from_rgb(245, 93, 93))
    }
    fn record_url_field(self, record: &RecordDto) -> Self {
        self.custom_field("議事録チケット", record.url(), false)
    }

    fn custom_field<T, U>(self, name: T, value: U, inline: bool) -> Self
    where
        T: ToString,
        U: ToString,
    {
        self.custom_fields([(name, value, inline)])
    }

    fn custom_fields<T, U, It>(self, fields: It) -> Self
    where
        T: ToString,
        U: ToString,
        It: IntoIterator<Item = (T, U, bool)>,
    {
        let fields = fields.into_iter().map(|(name, value, inline)| {
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

            (name, value, inline)
        });
        self.fields(fields)
    }
}
