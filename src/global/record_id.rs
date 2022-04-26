use crate::domain::id::IssueId;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

type IssueIdOpt = Option<IssueId>;

static RECORD_ID: Lazy<Arc<Mutex<IssueIdOpt>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

pub fn get() -> IssueIdOpt {
    *RECORD_ID.lock().unwrap()
}

pub fn update(id: IssueId) -> IssueIdOpt {
    *RECORD_ID.lock().unwrap() = Some(id);

    get()
}

pub fn clear() -> IssueIdOpt {
    *RECORD_ID.lock().unwrap() = None;

    get()
}

pub fn exists() -> bool {
    RECORD_ID.lock().unwrap().is_some()
}
