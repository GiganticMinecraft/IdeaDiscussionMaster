use super::InteractionResponse;
use crate::util::command::builder::{SlashCommandBuilder, SlashCommandOptionBuilder};
use serenity::model::interactions::application_command::ApplicationCommandOptionType;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new(
        "start",
        "アイデア会議を開始します。",
        Some(|map| {
            let n: i64 = map
                .get("discussion_issue_number")
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap();

            Ok(InteractionResponse::Message(format!(
                "会議が始まりました: 選択された議事録チケット: {}",
                n
            )))
        }),
    )
    .add_option(
        SlashCommandOptionBuilder::new(
            "discussion_issue_number",
            "議事録のチケット番号",
            ApplicationCommandOptionType::Integer,
            None,
        )
        .min_int(1)
        .required(true)
        .to_owned(),
    )
    .to_owned()
}
