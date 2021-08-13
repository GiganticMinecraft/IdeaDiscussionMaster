use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
    prelude::Context,
};
use std::collections::HashSet;

#[help]
#[individual_command_tip = "Helpコマンドです。\nコマンド名を引数として渡すことで、そのコマンドに関する特定の情報を得ることができます。"]
#[command_not_found_text = "Could not find: `{}`."]
// コマンドサジェストの精度指定。0で無効。
#[max_levenshtein_distance(3)]
pub async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;

    Ok(())
}
