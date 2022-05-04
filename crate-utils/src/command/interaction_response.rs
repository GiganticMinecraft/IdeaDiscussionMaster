use serenity::builder::CreateEmbed;

#[derive(Debug)]
pub enum InteractionResponse {
    Message(String),
    Messages(Vec<String>),
    Embed(CreateEmbed),
    Embeds(Vec<CreateEmbed>),
}
