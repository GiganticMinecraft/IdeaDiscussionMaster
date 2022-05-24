use derive_new::new;

#[derive(Debug, Clone, new, Default)]
pub struct Note {
    pub content: Vec<String>,
}

impl Note {
    pub fn from_string_content(content: String) -> Self {
        Self::new(content.split('\n').map(|s| s.to_owned()).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use itertools::Itertools;

    #[test]
    fn from_single_string_content() {
        let content = "content".to_string();
        assert_eq!(
            Note::from_string_content(content.clone()).content,
            vec![content]
        );
    }

    #[test]
    fn from_multi_string_content() {
        let contents = vec!["content", "content2", "content3"];
        let contents_str = contents.iter().join("\n");
        assert_eq!(Note::from_string_content(contents_str).content, contents);
    }
}
