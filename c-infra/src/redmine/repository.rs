mod agenda;
mod record;

use super::client::RedmineClient;

use std::marker::PhantomData;

pub struct RedmineRepositoryImpl<T> {
    pub client: RedmineClient,
    _marker: PhantomData<T>,
}

impl<T> RedmineRepositoryImpl<T> {
    pub fn new(url: String) -> Self {
        Self {
            client: RedmineClient::new(url),
            _marker: PhantomData::default(),
        }
    }
}
