mod start;
pub use start::start;

mod end;
pub use end::end;

mod vote;
pub use vote::vote;

mod agenda;
pub use agenda::agenda;

mod create;
pub use create::create;

pub type CommandResult = anyhow::Result<()>;
pub type Context<'a> = poise::Context<'a, crate::shared::Data, anyhow::Error>;
