mod secret_key;
use secret_key::SecretKey;

pub mod command;

mod create_embed_ext;
pub use create_embed_ext::CreateEmbedExt;

mod discussion;
pub use discussion::*;

mod hashset_ext;
pub use hashset_ext::HashSetExt;

mod env;
pub use env::Env;

pub use serenity::client::Context as SerenityContext;

pub const REDMINE_URL: &str = "https://redmine.seichi.click";
pub const GITHUB_URL: &str = "https://api.github.com/repos/GiganticMinecraft/SeichiAssist/issues";
