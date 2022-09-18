use derive_new::new;

#[derive(new)]
pub struct CreateNoteParam {
    pub contents: Vec<String>,
}

impl CreateNoteParam {
    pub fn from_single_line_string(contents: String) -> Self {
        Self::new(contents.split('\n').map(|s| s.to_owned()).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use itertools::Itertools;

    #[test]
    fn from_single_line_string() {
        let contents = "contents".to_string();
        assert_eq!(
            CreateNoteParam::from_single_line_string(contents.clone()).contents,
            vec![contents]
        );
    }

    #[test]
    fn from_multi_line_string() {
        let contents = vec!["contents", "contents2", "contents3"];
        let joined_contents = contents.iter().join("\n");
        assert_eq!(
            CreateNoteParam::from_single_line_string(joined_contents).contents,
            contents
        );
    }
}
