use crate::utils::secret_key::SecretKey;
use std::env;

pub struct Env {
    pub discord_token: String,
    pub redmine_api_key: String,
    pub discord_executor_role_id: u64,
    pub github_app_id: u64,
    pub github_secret_key: SecretKey,
}

impl Env {
    pub fn new() -> Self {
        let discord_token =
            env::var("DISCORD_TOKEN").expect("DiscordのBot Tokenが指定されていません");
        let redmine_api_key =
            env::var("REDMINE_KEY").expect("RedmineのAPIキーが指定されていません");
        let discord_executor_role_id = env::var("EXECUTABLE_ROLE_ID")
            .ok()
            .and_then(|id| id.parse::<u64>().ok())
            .expect("DiscordのロールIDが指定されていないか形式が正しくありません");
        let github_app_id = env::var("GH_APP_ID")
            .ok()
            .and_then(|id| id.parse::<u64>().ok())
            .expect("GitHubAppのIDが指定されていないか形式が正しくありません");
        let github_secret_key = SecretKey::new(env::var("GH_APP_RSA_KEY_PATH").ok())
            .expect("GitHubAppの秘密鍵ファイルが見つからないかファイルではありません");

        Self {
            discord_token,
            redmine_api_key,
            discord_executor_role_id,
            github_app_id,
            github_secret_key,
        }
    }
}
