mod start;
pub use start::start;

mod end;
pub use end::end;

mod vote;
pub use vote::vote;

pub type CommandResult = anyhow::Result<()>;
pub type Context<'a> = poise::Context<'a, crate::shared::Data, anyhow::Error>;
