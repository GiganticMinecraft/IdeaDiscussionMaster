mod register;
pub use register::register;

pub type CommandResult = anyhow::Result<()>;
pub type Context<'a> = poise::Context<'a, crate::shared::Data, anyhow::Error>;
