use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Env {
    pub discord_token: String,
    pub discord_application_id: u64,
    pub discord_guild_id: u64,
    pub redmine_api_key: String,
    pub gh_app_id: u64,
    pub gh_rsa_key_path: PathBuf,
}

impl Env {
    pub fn new() -> Self {
        envy::from_env::<Self>().expect("必要な環境変数を取得できませんでした。")
    }
}

impl std::default::Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[test]
    #[skip]
    fn success() {
        env::set_var("discord_token", "discord_token");
        env::set_var("discord_application_id", "1000");
        env::set_var("discord_guild_id", "1000");
        env::set_var("redmine_api_key", "redmine_api_key");
        env::set_var("github_app_id", "1000");
        env::set_var("github_secret_key", "github_secret_key");

        Env::new();
    }

    #[test]
    #[should_panic]
    #[skip]
    fn failure() {
        env::set_var("discord_application_id", "discord_application_id");

        Env::new();
    }
}
