use serenity::builder::CreateEmbed;

#[derive(Debug)]
pub enum InteractionResponse {
    Message(String),
    Embed(CreateEmbed),
}
