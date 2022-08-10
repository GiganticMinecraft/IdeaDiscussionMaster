mod agenda;
mod record;

#[derive(derive_new::new)]
pub struct RedmineRepositoryImpl<T> {
    pub client: crate::client::RedmineClient,
    marker: std::marker::PhantomData<T>,
}
