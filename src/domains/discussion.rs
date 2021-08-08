use serenity::prelude::Context;
use std::sync::atomic::Ordering;

use crate::globals::{agendas, current_agenda_id};

pub async fn go_to_next_agenda(ctx: &Context) -> Option<u16> {
    let agenda_id = {
        let cached_agendas = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<agendas::Agendas>()
                .expect("Expected Agendas in TypeMap.")
                .clone()
        };
        let agendas = cached_agendas.read().await;

        agendas
            .iter()
            .find(|(_, &status)| status == agendas::AgendaStatus::New)
            .map(|(id, _)| id.to_owned())
    };

    if agenda_id.is_some() {
        let cached_current_agenda_id = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<current_agenda_id::CurrentAgendaId>()
                .expect("Expected Agendas in TypeMap.")
                .clone()
        };

        cached_current_agenda_id.store(agenda_id.unwrap(), Ordering::Relaxed);
    }

    agenda_id
}
