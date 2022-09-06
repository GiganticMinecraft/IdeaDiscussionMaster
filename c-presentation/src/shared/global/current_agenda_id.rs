use c_domain::id::AgendaId;

use derive_new::new;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

#[derive(new)]
pub struct GlobalCurrentAgendaId(#[new(default)] Lazy<Arc<Mutex<Option<AgendaId>>>>);

impl GlobalCurrentAgendaId {
    pub fn save(&self, id: AgendaId) {
        let mut lock = self.0.lock().unwrap();
        *lock = Some(id)
    }

    pub fn clear(&self) {
        let mut lock = self.0.lock().unwrap();
        *lock = None
    }

    pub fn get(&self) -> Option<AgendaId> {
        self.0.lock().unwrap().clone()
    }
}
