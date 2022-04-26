use serenity::builder::CreateEmbed;

pub enum InteractionResponse {
    Message(String),
    Embed(CreateEmbed),
}
