use std::env;

pub struct Env {
    pub discord_token: String,
    pub redmine_api_key: String,
    pub discord_executor_role_id: String,
    pub github_app_id: String,
    pub github_secret_key_path: String,
}

impl Env {
    pub fn new() -> Self {
        let path_str = env::var("GH_APP_RSA_KEY_PATH")
            .expect("GitHubAppの秘密鍵へのファイルパスが見つかりません");
        let path = std::path::Path::new(&path_str);
        if !path.exists() || !path.is_file() {
            panic!("指定されたGitHubAppの秘密鍵は存在しません")
        }

        Self {
            discord_token: env::var("DISCORD_TOKEN").expect("DiscordのBot Tokenが見つかりません"),
            redmine_api_key: env::var("REDMINE_KEY").expect("RedmineのAPIキーが見つかりません"),
            discord_executor_role_id: env::var("EXECUTABLE_ROLE_ID")
                .expect("コマンドを実行できるDiscordのロールIDが見つかりません"),
            github_app_id: env::var("GH_APP_ID").expect("GitHubAppのIDが見つかりません"),
            github_secret_key_path: env::var("GH_APP_RSA_KEY_PATH")
                .expect("GitHubAppの秘密鍵へのファイルパスが見つかりません"),
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
