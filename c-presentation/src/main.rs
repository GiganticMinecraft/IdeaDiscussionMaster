use c_presentation::{commands, serenity, shared::Data};
use crate_shared::Env;

use poise::PrefixFrameworkOptions;

fn setup_logger() -> Result<(), fern::InitError> {
    let mut config = fern::Dispatch::new();
    config = config
        .level(log::LevelFilter::Info)
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
    setup_logger().expect("ログの初期化に失敗しました。");

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
            ..Default::default()
        })
        .token(Env::new().discord_token)
        .intents(
            serenity::GatewayIntents::non_privileged()
                .union(serenity::GatewayIntents::MESSAGE_CONTENT),
        )
        .user_data_setup(move |_, _, _| {
            // FIXME: fix url
            Box::pin(async move { Ok(Data::new("redmine".to_string())) })
        });

    framework.run().await.unwrap();
}
