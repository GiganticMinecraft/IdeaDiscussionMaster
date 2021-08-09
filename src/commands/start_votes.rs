use serenity::framework::standard::{macros::command, CommandResult};
use serenity::{model::prelude::Message, prelude::Context};

use crate::globals::current_agenda_id;

#[command]
#[aliases("svo")]
pub async fn start_votes(ctx: &Context, message: &Message) -> CommandResult {
    let current_agenda_id = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<current_agenda_id::CurrentAgendaId>()
            .expect("Expected CurrentAgendaId in TypeMap.")
            .clone()
    };

    Ok(())
}