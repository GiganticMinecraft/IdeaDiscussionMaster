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

// https://stackoverflow.com/questions/66769143/rust-passing-async-function-pointers

use application_interaction::ApplicationInteractions;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

pub type ArgsMap = HashMap<String, ApplicationInteractions>;
pub type CommandResult = anyhow::Result<InteractionResponse>;
pub type Executor = Arc<
    Box<
        dyn Fn(
                ArgsMap,
                super::SerenityContext,
            ) -> Pin<Box<dyn Future<Output = CommandResult> + Send + Sync>>
            + Send
            + Sync,
    >,
>;

pub fn force_boxed<T>(f: fn(ArgsMap, super::SerenityContext) -> T) -> Executor
where
    T: Future<Output = CommandResult> + 'static + Send + Sync,
{
    Arc::new(Box::new(move |map, context| Box::pin(f(map, context))))
}
