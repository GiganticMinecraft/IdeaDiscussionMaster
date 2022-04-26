use super::InteractionResponse;
use crate::util::command::builder::{
    SlashCommandBuilder, SlashCommandChoice, SlashCommandOptionBuilder,
};
use serenity::model::interactions::application_command::ApplicationCommandOptionType;

pub fn builder() -> SlashCommandBuilder {
    SlashCommandBuilder::new("agenda", "議題の操作を行います。", None)
        .add_option(
            SlashCommandOptionBuilder::new(
                "add",
                "議題を追加します。",
                ApplicationCommandOptionType::SubCommand,
                Some(|_map| Ok(InteractionResponse::Message("".to_string()))), // TODO: .messageや.embedをできるように
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "idea_issue_number",
                    "追加する議題のチケット番号",
                    ApplicationCommandOptionType::Integer,
                    None,
                )
                .min_int(1)
                .required(true),
            )
            .to_owned(),
        )
        .add_option(
            SlashCommandOptionBuilder::new(
                "list",
                "議題の一覧を表示します。",
                ApplicationCommandOptionType::SubCommand,
                Some(|_map| Ok(InteractionResponse::Message("".to_string()))),
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "status",
                    "変更後のステータス",
                    ApplicationCommandOptionType::Integer,
                    None,
                )
                .add_choice(("Approved", SlashCommandChoice::Int(1)))
                .add_choice(("Declined", SlashCommandChoice::Int(2)))
                .required(true),
            )
            .to_owned(),
        )
        .add_option(
            SlashCommandOptionBuilder::new(
                "set",
                "議題のステータスを変更します。",
                ApplicationCommandOptionType::SubCommand,
                Some(|_map| Ok(InteractionResponse::Message("".to_string()))),
            )
            .add_option(
                SlashCommandOptionBuilder::new(
                    "status",
                    "変更後のステータス",
                    ApplicationCommandOptionType::Integer,
                    None,
                )
                .add_choice(("Approved", SlashCommandChoice::Int(1)))
                .add_choice(("Declined", SlashCommandChoice::Int(2)))
                .required(true),
            )
            .to_owned(),
        )
        .to_owned()
}
