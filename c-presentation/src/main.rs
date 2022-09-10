use c_presentation::{commands, serenity, shared::Data};
use crate_shared::Env;

use argh::FromArgs;
use log::{debug, error, info};
use poise::{FrameworkError, PrefixFrameworkOptions};

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

    let commands = vec![commands::register(), commands::start(), commands::end()];
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
                    if let FrameworkError::Command { error, ctx } = err {
                        let message = format!(
                            "コマンド(/{})の処理中にエラーが発生しました: {:#?}",
                            ctx.command().qualified_name,
                            error
                        );

                        error!("{}", message);
                        let _ = ctx.say(message).await;
                    }
                })
            },
            ..Default::default()
        })
        .token(Env::new().discord_token)
        .intents(
            serenity::GatewayIntents::non_privileged()
                .union(serenity::GatewayIntents::MESSAGE_CONTENT),
        )
        .user_data_setup(move |_, _, _| {
            Box::pin(async move { Ok(Data::new("https://redmine.seichi.click".to_string())) })
        });

    framework.run().await.unwrap();
}
