use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::{
    domains::discord_embed,
    globals::{agendas, record_id},
};

#[command]
#[usage = "(引数なし)"]
#[aliases("svo")]
#[description = "投票を開始するコマンドです。"]
pub async fn start_votes(ctx: &Context, message: &Message) -> CommandResult {
    let record_id = record_id::read(ctx).await;
    let current_agenda_id = agendas::find_current_agenda_id(ctx).await;
    let description = vec![
        "提起されている議題についての採決を行います。",
        "以下のリアクションで投票を行ってください。過半数を超え次第、次の議題へと移ります。",
        ":o:: 承認",
        ":x:: 却下",
        "",
        "※リアクションがすべて表示されてからリアクションを行わないと、投票が無効になる場合があります。",
    ]
    .join("\n");
    let voted_message = message
        .channel_id
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                if current_agenda_id.is_some() {
                    discord_embed::default_colored_embed(embed, record_id)
                        .title(format!("採決: #{}", current_agenda_id.unwrap()))
                        .description(description)
                } else {
                    discord_embed::default_failure_embed(embed, record_id)
                        .title("現在進行中の議題はありません")
                }
            })
        })
        .await?;

    if current_agenda_id.is_some() {
        voted_message.react(&ctx.http, '⭕').await?;
        voted_message.react(&ctx.http, '❌').await?;

        agendas::update_votes_message_id(ctx, current_agenda_id.unwrap(), Some(voted_message.id))
            .await;
    }

    Ok(())
}
