use anyhow::ensure;
use serenity::model::interactions::application_command::ApplicationCommandInteractionDataOptionValue as OptionValue;

#[derive(Debug, Clone)]
pub enum SlashCommand {
    Command(String),
    SubCommand(String),
    Option(Box<OptionValue>),
}

#[derive(Debug, Clone)]
pub enum ApplicationInteractions {
    SlashCommand(SlashCommand),
}

impl TryInto<String> for ApplicationInteractions {
    type Error = anyhow::Error;
    fn try_into(self) -> anyhow::Result<String> {
        if let ApplicationInteractions::SlashCommand(SlashCommand::Option(b)) = self {
            if let OptionValue::String(v) = *b {
                return Ok(v);
            }
        }

        anyhow::bail!("Can't convert this interaction to String")
    }
}

impl TryInto<i64> for ApplicationInteractions {
    type Error = anyhow::Error;
    fn try_into(self) -> anyhow::Result<i64> {
        if let ApplicationInteractions::SlashCommand(SlashCommand::Option(b)) = self {
            if let OptionValue::Integer(v) = *b {
                return Ok(v);
            }
        }

        anyhow::bail!("Can't convert this interaction to Integer")
    }
}

impl TryInto<u16> for ApplicationInteractions {
    type Error = anyhow::Error;
    fn try_into(self) -> anyhow::Result<u16> {
        if let ApplicationInteractions::SlashCommand(SlashCommand::Option(b)) = self {
            if let OptionValue::Integer(v) = *b {
                ensure!(v >= u16::MIN.into(), "The arg is too low value: {}", v);
                ensure!(v <= u16::MAX.into(), "The arg is too high value: {}", v);

                return Ok(v as u16);
            }
        }

        anyhow::bail!("Can't convert this interaction to Integer")
    }
}
