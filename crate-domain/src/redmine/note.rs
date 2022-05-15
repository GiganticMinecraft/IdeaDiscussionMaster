use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct Note {
    pub content: Vec<String>,
}

impl Note {
    pub fn from_string_content(content: String) -> Self {
        Self::new(content.split('\n').map(|s| s.to_owned()).collect())
    }
}
