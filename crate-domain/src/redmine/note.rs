use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct Note {
    pub author: String,
    pub content: Vec<String>,
}

impl Note {
    pub fn from_string_content(author: String, content: String) -> Self {
        Self::new(author, content.split('\n').map(|s| s.to_owned()).collect())
    }
}
