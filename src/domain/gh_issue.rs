use derive_new::new;

#[derive(new, Clone)]
pub struct Issue {
    pub title: String,
    pub content: String,
    pub labels: Vec<String>,
}
