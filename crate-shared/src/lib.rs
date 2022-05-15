pub mod command;
pub mod ext;

mod discussion;
pub use discussion::*;

mod env;
pub use env::Env;

mod secret_key;
use secret_key::SecretKey;

pub use serenity::client::Context as SerenityContext;

pub const REDMINE_URL: &str = "https://redmine.seichi.click";
pub const GITHUB_URL: &str = "https://api.github.com/repos/GiganticMinecraft/SeichiAssist/issues";
