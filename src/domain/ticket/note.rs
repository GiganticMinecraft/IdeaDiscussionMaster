#[derive(Debug, Clone)]
pub struct Note {
    pub author: String,
    pub content: Vec<String>,
}

impl Note {
    pub fn new(author: String, content: Vec<String>) -> Self {
        Self { author, content }
    }

    pub fn from_string_content(author: String, content: String) -> Self {
        Self {
            author,
            content: content.split('\n').map(|s| s.to_owned()).collect(),
        }
    }
}
