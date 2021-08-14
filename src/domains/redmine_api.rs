cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use crate::domains::redmine_client::MockRedmineClient as RedmineClient;
    } else {
        pub use crate::domains::redmine_client::RedmineClient;
    }
}

use crate::domains::{agenda_status, custom_error, redmine};

pub const REDMINE_URL: &str = "https://redmine.seichi.click";

pub struct RedmineApi {
    client: RedmineClient,
}

impl RedmineApi {
    pub fn new(client: RedmineClient) -> Self {
        RedmineApi { client }
    }

    pub async fn fetch_issue(
        &self,
        issue_id: &u16,
    ) -> Result<redmine::RedmineIssue, custom_error::Error> {
        self.client.fetch_issue(issue_id).await
    }

    pub async fn fetch_issue_with_relations(
        &self,
        issue_id: &u16,
    ) -> Result<redmine::RedmineIssue, custom_error::Error> {
        self.client.fetch_issue_with_relations(issue_id).await
    }

    pub async fn update_issue_status(&self, issue_id: &u16, status: &agenda_status::AgendaStatus) {
        self.client.update_issue_status(issue_id, status).await
    }
}
