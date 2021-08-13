use serenity::{
    async_trait,
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
    domains::{agenda_status, discord_embed, discussion, redmine_api},
    globals::{agendas, current_agenda_id, record_id, voice_chat_channel_id, voted_message_id},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let voted_message_id = voted_message_id::read(&ctx).await;
        if let Ok(user) = reaction.user(&ctx.http).await {
            if user.bot {
                return;
            }
        }
        if voted_message_id == 0 {
            return;
        }

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
            num.len() - 1
        } else {
            return;
        };
        // TODO: 過半数を超えていたら以下の操作をする
        // redmineのステータス変更

        if reaction_counts <= half_of_vc_members {
            return;
        }

        voted_message_id::clear(&ctx).await;

        let record_id = record_id::read(&ctx).await;
        let current_agenda_id = current_agenda_id::read(&ctx).await;

        let _ = reaction
            .channel_id
            .send_message(&ctx.http, |msg| {
                msg.embed(|embed| {
                    discord_embed::votes_result_embed(
                        embed,
                        record_id,
                        current_agenda_id,
                        status_reaction,
                    )
                })
            })
            .await;

        agendas::write(&ctx, current_agenda_id, status_reaction).await;
        current_agenda_id::clear(&ctx).await;

        let next_agenda_id = discussion::go_to_next_agenda(&ctx).await;
        let redmine_api = redmine_api::RedmineApi::new(RedmineClient::new());
        let next_redmine_issue = redmine_api
            .fetch_issue(&next_agenda_id.unwrap_or_default())
            .await
            .ok();
        let _ = reaction
            .channel_id
            .send_message(&ctx.http, |msg| {
                msg.embed(|embed| {
                    discord_embed::next_agenda_embed(embed, record_id, next_redmine_issue)
                })
            })
            .await;
    }
}
