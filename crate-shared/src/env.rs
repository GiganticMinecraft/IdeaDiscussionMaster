use super::SecretKey;
use std::{env, path::PathBuf};

pub struct Env {
    pub discord_token: String,
    pub discord_application_id: u64,
    pub redmine_api_key: String,
    pub github_app_id: u64,
    pub github_secret_key: PathBuf,
}

impl Env {
    pub fn new() -> Self {
        let discord_token =
            env::var("DISCORD_TOKEN").expect("DiscordのBot Tokenが指定されていません");
        let discord_application_id = env::var("DISCORD_APPLICATION_ID")
            .ok()
            .and_then(|id| id.parse::<u64>().ok())
            .expect("DiscordBotのApplication IDが指定されていないか形式が正しくありません");
        let redmine_api_key =
            env::var("REDMINE_KEY").expect("RedmineのAPIキーが指定されていません");
        let github_app_id = env::var("GH_APP_ID")
            .ok()
            .and_then(|id| id.parse::<u64>().ok())
            .expect("GitHubAppのIDが指定されていないか形式が正しくありません");
        let github_secret_key = SecretKey::new(env::var("GH_APP_RSA_KEY_PATH").ok())
            .expect("GitHubAppの秘密鍵ファイルが見つからないかファイルではありません")
            .0;

        Self {
            discord_token,
            discord_application_id,
            redmine_api_key,
            github_app_id,
            github_secret_key,
        }
    }
}

impl std::default::Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
