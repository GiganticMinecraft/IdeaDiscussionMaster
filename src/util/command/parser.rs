use super::application_interaction::{ApplicationInteractions, SlashCommandType};
use serenity::model::interactions::application_command::{
    ApplicationCommandInteractionData, ApplicationCommandInteractionDataOption,
    ApplicationCommandOptionType,
};

pub type ParsedCmdData = Vec<(String, ApplicationInteractions)>;

pub trait Parser {
    fn parse(&self) -> anyhow::Result<ParsedCmdData>;
}

impl Parser for ApplicationCommandInteractionData {
    fn parse(&self) -> anyhow::Result<ParsedCmdData> {
        let mut items = vec![(
            "command".to_string(),
            ApplicationInteractions::SlashCommand(SlashCommandType::Command(self.name.clone())),
        )];

        struct Parser<'a> {
            parser: ParserImpl<'a>,
        }

        type ParserImpl<'a> = &'a dyn Fn(
            &Parser,
            &mut ParsedCmdData,
            &Vec<ApplicationCommandInteractionDataOption>,
        ) -> anyhow::Result<ParsedCmdData>;

        let parser = Parser {
            parser: &|p, array, options| {
                if options.is_empty() {
                    Ok(array.clone())
                } else {
                    for o in options {
                        type OptionType = ApplicationCommandOptionType;

                        match o.kind {
                            OptionType::SubCommand => array.push((
                                "sub_command".to_string(),
                                ApplicationInteractions::SlashCommand(
                                    SlashCommandType::SubCommand(o.name.clone()),
                                ),
                            )),
                            OptionType::String
                            | OptionType::Integer
                            | OptionType::Boolean
                            | OptionType::Number
                            | OptionType::User
                            | OptionType::Role
                            | OptionType::Channel
                            | OptionType::Mentionable => array.push((
                                o.name.clone(),
                                ApplicationInteractions::SlashCommand(SlashCommandType::Option(
                                    Box::new(o.resolved.as_ref().unwrap().clone()),
                                )),
                            )),
                            unknown => {
                                anyhow::bail!("Invalid option type: {:?}", unknown);
                            }
                        }
                    }

                    match options.last() {
                        Some(last) => (p.parser)(p, array, &last.options),
                        None => Ok(array.clone()),
                    }
                }
            },
        };

        (parser.parser)(&parser, &mut items, &self.options)
    }
}
