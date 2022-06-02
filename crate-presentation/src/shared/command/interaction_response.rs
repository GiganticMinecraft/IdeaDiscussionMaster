use crate_shared::ext::CreateEmbedExt;

use serenity::builder::CreateEmbed;

#[allow(dead_code)]
#[derive(Debug)]
pub enum InteractionResponse {
    Message(String),
    Messages(Vec<String>),
    Embed(CreateEmbed),
    Embeds(Vec<CreateEmbed>),
}

impl InteractionResponse {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Message(msg) => msg.is_empty(),
            Self::Messages(array) => array.iter().any(|str| str.is_empty()),
            Self::Embed(embed) => embed.is_empty(),
            Self::Embeds(embeds) => embeds.iter().any(|embed| embed.is_empty()),
        }
    }
}

impl Default for InteractionResponse {
    fn default() -> Self {
        Self::Message("Success: There is no message".to_string())
    }
}
