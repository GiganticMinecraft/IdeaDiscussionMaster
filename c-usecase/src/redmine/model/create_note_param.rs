use c_domain::redmine::model::Note;

use derive_new::new;

#[derive(new)]
pub struct CreateNoteParam {
    pub contents: String,
}

impl CreateNoteParam {
    pub fn from_multi_line_string(contents: Vec<String>) -> Self {
        Self::new(contents.join("\n"))
    }
}

impl From<CreateNoteParam> for Note {
    fn from(param: CreateNoteParam) -> Self {
        Self::new(param.contents)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_single_line_string() {
        let contents = vec!["contents".to_string()];
        assert_eq!(
            &CreateNoteParam::from_multi_line_string(contents.clone()).contents,
            contents.first().unwrap()
        );
    }

    #[test]
    fn from_multi_line_string() {
        let contents: Vec<_> = vec!["contents", "contents2", "contents3"]
            .into_iter()
            .map(|str| str.to_string())
            .collect();
        assert_eq!(
            CreateNoteParam::from_multi_line_string(contents.clone()).contents,
            contents.join("\n")
        );
    }
}
