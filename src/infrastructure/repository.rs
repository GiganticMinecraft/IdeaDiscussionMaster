pub mod agenda;
pub mod gh_issue;
pub mod record;

use super::persistence::{GitHub, Redmine};
use derive_new::new;
use std::marker::PhantomData;

#[derive(new)]
pub struct RedminePersistenceImpl<T> {
    pub client: Redmine,
    _marker: PhantomData<T>,
}

#[derive(new)]
pub struct GitHubPersistenceImpl<T> {
    pub client: GitHub,
    _marker: PhantomData<T>,
}
