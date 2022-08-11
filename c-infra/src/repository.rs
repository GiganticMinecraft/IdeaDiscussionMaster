mod agenda;
mod record;

use crate::client::RedmineClient;
use std::marker::PhantomData;

pub struct RedmineRepositoryImpl<T> {
    pub client: RedmineClient,
    marker: std::marker::PhantomData<T>,
}

impl<T> RedmineRepositoryImpl<T> {
    pub fn new(url: String) -> Self {
        Self {
            client: RedmineClient::new(url),
            marker: PhantomData::default(),
        }
    }
}
