use derive_new::new;

#[derive(new)]
pub struct Issue {
    pub title: String,
    pub content: String,
    pub labels: Vec<String>,
}
