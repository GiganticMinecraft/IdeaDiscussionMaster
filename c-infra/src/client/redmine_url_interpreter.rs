use crate_shared::Env;

pub struct RedmineUrlInterpreter {
    /// `http(s)://`を含んでいるRedmineのURL
    url: String,
    token: String,
}

impl RedmineUrlInterpreter {
    pub fn new(url: String) -> Self {
        Self {
            url,
            token: format!("key={}", Env::new().redmine_api_key),
        }
    }

    pub fn issue_url(&self, id: u16) -> String {
        format!("{}/issues/{}.json?{}", self.url, id, self.token)
    }

    pub fn issues_url(&self) -> String {
        format!("{}/issues.json?{}", self.url, self.token)
    }

    pub fn issue_relations_url(&self, id: u16) -> String {
        format!("{}/issues/{}/relations.json?{}", self.url, id, self.token)
    }
}
