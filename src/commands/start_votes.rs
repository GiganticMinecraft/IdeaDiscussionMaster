use crate::{
    commands::end_votes,
    domains::status::AgendaStatus,
    globals::{agendas, record_id, voice_chat_channel_id},
    utils::{discord_embed, discussion},
};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult, Delimiter},
    model::{channel::ReactionType, prelude::Message},
    prelude::Context,
};
use std::str::FromStr;
use tokio::time::{self, Duration};

#[command]
#[usage = "(引数なし)"]
#[aliases("svo")]
#[description = "投票を開始するコマンドです。"]
pub async fn start_votes(ctx: &Context, message: &Message) -> CommandResult {
    let current_agenda_id = agendas::find_current_agenda_id(ctx).await;
    // 議題がないorすでに採決中ならば処理を終了
    if current_agenda_id.is_none() {
        return Err("現在進行中の議題はありません".into());
    } else if agendas::find_votes_message_id(ctx, current_agenda_id.unwrap())
        .await
        .is_some()
    {
        return Err("すでに採決が行われています".into());
    };
    let current_agenda_id = current_agenda_id.unwrap();

    let description = vec![
        "提起されている議題についての採決を行います。",
        "以下のリアクションで投票を行ってください。過半数を超え次第、次の議題へと移ります。",
        ":o:: 承認",
        ":x:: 却下",
        "",
        "※リアクションがすべて表示されてからリアクションを行わないと、投票が無効になる場合があります。",
    ]
    .join("\n");
    let record_id = record_id::read(ctx).await.unwrap();
    let voted_message = message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                discord_embed::default_colored_embed(embed, record_id)
                    .title(format!("採決: #{}", current_agenda_id))
                    .description(description)
            })
        })
        .await?;
    voted_message.react(&ctx.http, '⭕').await?;
    voted_message.react(&ctx.http, '❌').await?;

    agendas::update_votes_message_id(ctx, current_agenda_id, Some(voted_message.id)).await;

    let vc_id = voice_chat_channel_id::read(ctx).await.unwrap();
    loop {
        // end_votesコマンド等で議題が次に行っている場合処理を終了させないと永遠にループする
        if agendas::find_current_agenda_id(ctx).await != Some(current_agenda_id) {
            break;
        }

        let vc_members = discussion::fetch_voice_states(ctx, message.guild_id)
            .await
            .iter()
            .filter(|(_, state)| state.channel_id.unwrap_or_default() == vc_id)
            .count();

        if let Some(status) = get_votes_result(ctx, &voted_message, vc_members).await {
            // end_votesコマンドを強制的に叩く
            let _ = end_votes::end_votes(
                ctx,
                &voted_message,
                Args::new(&status.ja(), &[Delimiter::Single(' ')]),
            )
            .await;
            break;
        };

        time::sleep(Duration::from_secs(2)).await;
    }

    Ok(())
}

async fn get_votes_result(
    ctx: &Context,
    voted_message: &Message,
    vc_members: usize,
) -> Option<AgendaStatus> {
    for status in AgendaStatus::done_statuses() {
        if let Ok(users) = ctx
            .http
            .as_ref()
            .get_reaction_users(
                voted_message.channel_id.as_u64().to_owned(),
                voted_message.id.as_u64().to_owned(),
                &ReactionType::from_str(&status.emoji()).unwrap(),
                100,
                None,
            )
            .await
        {
            if users.len() - 1 > vc_members / 2 {
                return Some(status);
            }
        };
    }

    None
}
