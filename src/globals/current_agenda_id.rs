use serenity::prelude::{Context, TypeMapKey};
use std::sync::{atomic::{AtomicU16, Ordering}, Arc};

#[deprecated]
pub struct CurrentAgendaId;

#[allow(deprecated)]
impl TypeMapKey for CurrentAgendaId {
    type Value = Arc<AtomicU16>;
}

#[allow(deprecated)]
pub async fn read(ctx: &Context) -> u16 {
    let cached_current_agenda_id = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<CurrentAgendaId>()
            .expect("Expected CurrentAgendaId in TypeMap.")
            .clone()
    };
    cached_current_agenda_id.load(Ordering::Relaxed)
}

#[allow(deprecated)]
pub async fn write(ctx: &Context, new_agenda_id: u16) -> u16 {
    let cached_current_agenda_id = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<CurrentAgendaId>()
            .expect("Expected CurrentAgendaId in TypeMap.")
            .clone()
    };
    cached_current_agenda_id.store(new_agenda_id, Ordering::Relaxed);
    new_agenda_id
}

pub async fn clear(ctx: &Context) {
    write(ctx, 0).await;
}
