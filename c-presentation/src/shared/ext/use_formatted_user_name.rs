use poise::serenity_prelude::User;

pub trait UseFormattedUserName {
    fn formatted_user_name(&self) -> String;
}

impl UseFormattedUserName for User {
    fn formatted_user_name(&self) -> String {
        format!("{}#{}", self.name, self.discriminator)
    }
}
