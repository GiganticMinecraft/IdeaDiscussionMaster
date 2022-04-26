pub mod application_interaction;
pub mod builder;

mod interaction_response;
pub use interaction_response::InteractionResponse;

mod command_ext;
pub use command_ext::CommandExt;

mod parser;
pub use parser::Parser;

mod slash_command_choice;
pub use slash_command_choice::SlashCommandChoice;

pub type Executor = fn(
    std::collections::HashMap<String, application_interaction::ApplicationInteractions>,
) -> anyhow::Result<InteractionResponse>;
