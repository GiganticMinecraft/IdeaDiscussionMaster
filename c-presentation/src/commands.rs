mod register;
pub use register::register;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, crate::shared::Data, Error>;
