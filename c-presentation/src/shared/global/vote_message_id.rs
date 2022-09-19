use derive_new::new;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

#[derive(new)]
pub struct GlobalVoteMessageId(#[new(default)] Lazy<Arc<Mutex<Option<u64>>>>);

impl GlobalVoteMessageId {
    pub fn save(&self, id: u64) {
        let mut lock = self.0.lock().unwrap();
        *lock = Some(id)
    }

    pub fn clear(&self) {
        let mut lock = self.0.lock().unwrap();
        *lock = None
    }

    pub fn get(&self) -> Option<u64> {
        *self.0.lock().unwrap()
    }
}
