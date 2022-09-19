use c_domain::github::model::Issue;
use derive_new::new;

#[derive(new)]
pub struct CreateIssueParam {
    pub title: String,
    pub content: String,
    pub labels: Vec<String>,
}

impl From<CreateIssueParam> for Issue {
    fn from(param: CreateIssueParam) -> Self {
        Self::new(param.title, param.content, param.labels)
    }
}
