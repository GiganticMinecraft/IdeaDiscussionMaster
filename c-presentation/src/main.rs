use c_presentation::{commands, shared::Data};
use crate_shared::Env;

use argh::FromArgs;
use itertools::Itertools;
use log::{debug, error, info};
use poise::{
    builtins::create_application_commands,
    serenity_prelude::{Context, GatewayIntents, GuildId},
    Event, FrameworkContext, FrameworkError, PrefixFrameworkOptions,
};

#[derive(FromArgs)]
/// CLI arg
struct Arg {
    #[argh(switch, short = 'v')]
    /// whether or not to log debug
    is_verbose: bool,
}

fn setup_logger(is_verbose: bool) -> Result<(), fern::InitError> {
    let mut config = fern::Dispatch::new();

    let crate_log_level = if is_verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    config = config
        .level(log::LevelFilter::Info)
        .level_for("c_domain", crate_log_level)
        .level_for("c_usecase", crate_log_level)
        .level_for("c_infra", crate_log_level)
        .level_for("c_presentation", crate_log_level)
        .level_for("surf", log::LevelFilter::Off)
        .level_for("serenity", log::LevelFilter::Off)
        .level_for("tracing::span", log::LevelFilter::Off);

    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout());

    config.chain(stdout_config).apply()?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let arg: Arg = argh::from_env();
    setup_logger(arg.is_verbose).expect("ログの初期化に失敗しました");
    if arg.is_verbose {
        debug!("Logging level is debug")
    };

    let commands = vec![
        commands::start(),
        commands::end(),
        commands::vote(),
        commands::agenda(),
        commands::create(),
    ];
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("\\".into()),
                ..Default::default()
            },
            command_check: Some(|ctx| {
                Box::pin(async move {
                    let is_in_guild = ctx.guild().is_some();
                    let is_user = !ctx.author().bot;

                    Ok(is_in_guild && is_user)
                })
            }),
            pre_command: |ctx| {
                Box::pin(async move {
                    info!(
                        "{}#{}さんがコマンド(/{})を実行しました",
                        ctx.author().name,
                        ctx.author().discriminator,
                        ctx.command().qualified_name
                    );
                    let _ = ctx.defer().await;
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    info!(
                        "{}#{}さんがコマンド(/{})を実行し成功しました",
                        ctx.author().name,
                        ctx.author().discriminator,
                        ctx.command().qualified_name
                    );
                })
            },
            on_error: |err: FrameworkError<_, anyhow::Error>| {
                Box::pin(async move {
                    match err {
                        FrameworkError::Command { error, ctx } => {
                            let message = format!(
                                "コマンド(/{})の処理中にエラーが発生しました: {:?}",
                                ctx.command().qualified_name,
                                error
                            );

                            error!("{}", message);
                            let _ = ctx.say(message).await;
                        }
                        FrameworkError::Listener {
                            error,
                            event: Event::Ready { .. },
                            ..
                        } => {
                            error!("Botの起動処理中にエラーが発生しました: {:?}", error);
                        }
                        _ => {}
                    };
                })
            },
            listener: |ctx: &Context, event, framework_ctx: FrameworkContext<_, _>, _| {
                Box::pin(async move {
                    if let Event::Ready { .. } = event {
                        // region register commands
                        let create_commands =
                            create_application_commands(&framework_ctx.options().commands);
                        let guild_id = GuildId::from(Env::new().discord_guild_id);
                        let registered_commands = guild_id
                            .set_application_commands(&ctx.http, |b| {
                                *b = create_commands;
                                b
                            })
                            .await?;
                        info!(
                            "以下のコマンドを登録しました: {}",
                            registered_commands
                                .into_iter()
                                .map(|cmd| cmd.name)
                                .collect_vec()
                                .join(", ")
                        );
                        // endregion

                        info!("Botが正常に起動しました");
                    }

                    Ok(())
                })
            },
            ..Default::default()
        })
        .token(Env::new().discord_token)
        .intents(GatewayIntents::non_privileged().union(GatewayIntents::MESSAGE_CONTENT))
        .user_data_setup(move |_, _, _| {
            Box::pin(async move { Ok(Data::new("https://redmine.seichi.click".to_string())) })
        });

    framework.run().await.unwrap();
}
