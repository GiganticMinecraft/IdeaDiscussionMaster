use c_presentation::{commands, serenity, shared::Data};
use crate_shared::Env;
use log::{error, info};

use poise::{FrameworkError, PrefixFrameworkOptions};

fn setup_logger() -> Result<(), fern::InitError> {
    let mut config = fern::Dispatch::new();
    config = config
        .level(log::LevelFilter::Info)
        .level_for("c_domain", log::LevelFilter::Debug)
        .level_for("c_usecase", log::LevelFilter::Debug)
        .level_for("c_infra", log::LevelFilter::Debug)
        .level_for("c_presentation", log::LevelFilter::Debug)
        .level_for("surf", log::LevelFilter::Off);

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
    setup_logger().expect("ログの初期化に失敗しました");

    let commands = vec![commands::register()];
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
                        ctx.command().name
                    );
                })
            },
            on_error: |err: FrameworkError<_, anyhow::Error>| {
                Box::pin(async move {
                    if let FrameworkError::Command { error, ctx } = err {
                        let message = format!(
                            "コマンド(/{})の処理中にエラーが発生しました: {:#?}",
                            ctx.command().name,
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
