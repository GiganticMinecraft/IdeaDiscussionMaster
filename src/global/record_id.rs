use crate::domain::id::RecordId;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

type RecordIdOpt = Option<RecordId>;

static RECORD_ID: Lazy<Arc<Mutex<RecordIdOpt>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

pub fn get() -> RecordIdOpt {
    *RECORD_ID.lock().unwrap()
}

pub fn update(id: RecordId) -> RecordIdOpt {
    *RECORD_ID.lock().unwrap() = Some(id);

    get()
}

pub fn clear() -> RecordIdOpt {
    *RECORD_ID.lock().unwrap() = None;

    get()
}

pub fn exists() -> bool {
    RECORD_ID.lock().unwrap().is_some()
}
