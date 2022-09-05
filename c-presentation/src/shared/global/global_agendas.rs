use super::global_agenda::GlobalAgenda;
use crate::shared::HashSetExt;
use c_domain::id::AgendaId;

use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

type GlobalAgendaSet = HashSet<GlobalAgenda>;

pub struct GlobalAgendas(Arc<Mutex<GlobalAgendaSet>>);

impl GlobalAgendas {
    pub fn new(hash_set: GlobalAgendaSet) -> Self {
        Self(Arc::new(Mutex::new(hash_set)))
    }

    pub fn add(&self, agenda: GlobalAgenda) -> GlobalAgendaSet {
        let mut set = self.0.lock().unwrap();
        set.insert(agenda);

        set.clone()
    }

    pub fn list(&self) -> GlobalAgendaSet {
        self.0.lock().unwrap().clone()
    }

    pub fn find_by_id(&self, id: AgendaId) -> Option<GlobalAgenda> {
        self.list().into_iter().find(|agenda| agenda.id == id)
    }

    pub fn save(&self, new_agenda: GlobalAgenda) {
        let old = self.find_by_id(new_agenda.id.clone());
        let mut set = self.0.lock().unwrap();

        if let Some(old) = old {
            if old == new_agenda {
                return;
            }
            set.update(&old, new_agenda);
        } else {
            set.insert(new_agenda);
        }
    }
}
