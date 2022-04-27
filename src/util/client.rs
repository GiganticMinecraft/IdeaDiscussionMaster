mod redmine_client;
pub use redmine_client::RedmineClient;

mod github_client;
pub use github_client::GitHubClient;

pub const REDMINE_URL: &str = "https://redmine.seichi.click";
pub const GITHUB_URL: &str = "https://api.github.com/repos/GiganticMinecraft/SeichiAssist/issues";
