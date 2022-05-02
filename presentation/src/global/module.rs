use crate::module::Module;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

static MODULE: Lazy<Arc<Mutex<Option<Module>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

pub async fn init() {
    let module = Module::new().await;

    *MODULE.lock().unwrap() = Some(module);
}

pub fn get() -> Option<Module> {
    MODULE.lock().unwrap().clone()
}
