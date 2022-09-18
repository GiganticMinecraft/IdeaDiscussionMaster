use derive_new::new;

#[derive(new)]
pub struct Note {
    pub contents: String,
}
