use serenity::{
    async_trait,
    framework::standard::{Args, Delimiter},
    model::{
        channel::{Reaction, ReactionType},
        gateway::Ready,
        id::ChannelId,
    },
    prelude::{Context, EventHandler},
};
use std::str::FromStr;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use crate::domains::redmine_client::MockRedmineClient as RedmineClient;
    } else {
        pub use crate::domains::redmine_client::RedmineClient;
    }
}

use crate::{
    commands::end_votes,
    domains::{discussion, status::agenda_status},
    globals::{agendas, voice_chat_channel_id},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Ok(user) = reaction.user(&ctx.http).await {
            if user.bot {
                return;
            }
        }
        let current_agenda_id = agendas::find_current_agenda_id(&ctx).await.unwrap();
        let voted_message_id = agendas::find_votes_message_id(&ctx, current_agenda_id).await;
        if voted_message_id.is_none() || voted_message_id.unwrap() != reaction.message_id {
            return;
        };

        let vc_id = voice_chat_channel_id::read(&ctx).await;
        let half_of_vc_members = discussion::fetch_voice_states(&ctx, reaction.guild_id)
            .await
            .iter()
            .filter(|(_, state)| state.channel_id.unwrap_or_default() == ChannelId(vc_id))
            .count()
            / 2;

        let status_reaction = if let Some(emoji) = agenda_status::AgendaStatus::done_statuses()
            .iter()
            .find(|status| reaction.emoji.unicode_eq(&status.emoji()))
        {
            emoji.to_owned()
        } else {
            return;
        };
        let reaction_counts = if let Ok(num) = ctx
            .http
            .as_ref()
            .get_reaction_users(
                reaction.channel_id.as_u64().to_owned(),
                reaction.message_id.as_u64().to_owned(),
                &ReactionType::from_str(&status_reaction.emoji()).unwrap(),
                100,
                None,
            )
            .await
        {
            // 本botのリアクションの分
            num.len() - 1
        } else {
            return;
        };

        if reaction_counts <= half_of_vc_members {
            return;
        }

        // end_votesコマンドを強制的に叩く
        let _ = end_votes::end_votes(
            &ctx,
            &reaction.message(&ctx.http).await.unwrap(),
            Args::new(&status_reaction.ja(), &[Delimiter::Single(' ')]),
        )
        .await;
    }
}
