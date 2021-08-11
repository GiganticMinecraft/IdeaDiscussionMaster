use serenity::prelude::Context;

use crate::globals::{agendas, current_agenda_id};

pub async fn go_to_next_agenda(ctx: &Context) -> Option<u16> {
    let agenda_id = {
        let cached_agendas = agendas::read(ctx).await;

        cached_agendas
            .iter()
            .find(|(_, &status)| status == agendas::AgendaStatus::New)
            .map(|(id, _)| id.to_owned())
    };

    if agenda_id.is_some() {
        current_agenda_id::write(ctx, agenda_id.unwrap()).await;
    }

    agenda_id
}
