mod secret_key;
use secret_key::SecretKey;

pub mod command;
pub mod discord_embed;
pub mod discussion;

mod hashset_ext;
pub use hashset_ext::HashSetExt;

mod env;
pub use env::Env;

pub const REDMINE_URL: &str = "https://redmine.seichi.click";
pub const GITHUB_URL: &str = "https://api.github.com/repos/GiganticMinecraft/SeichiAssist/issues";