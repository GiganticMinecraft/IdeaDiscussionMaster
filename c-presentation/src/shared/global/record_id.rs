use c_domain::id::RecordId;

use derive_new::new;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

#[derive(new)]
pub struct GlobalRecordId(#[new(default)] Lazy<Arc<Mutex<Option<RecordId>>>>);

impl GlobalRecordId {
    pub fn save(&self, id: RecordId) {
        let mut lock = self.0.lock().unwrap();
        *lock = Some(id)
    }

    pub fn clear(&self) {
        let mut lock = self.0.lock().unwrap();
        *lock = None
    }

    pub fn get(&self) -> Option<RecordId> {
        self.0.lock().unwrap().clone()
    }
}
