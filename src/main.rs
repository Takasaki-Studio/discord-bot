mod commands;
mod config;
mod events;
mod helpers;

use config::Config;

use poise::{
    serenity_prelude::GatewayIntents, Framework, FrameworkOptions, PrefixFrameworkOptions,
};

pub struct Data {
    pub config: Config,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let config = Config::get_configs();

    Framework::build()
        .token(&config.token)
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move { Ok(Data { config }) })
        })
        .options(FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(".".to_owned()),
                ..Default::default()
            },
            commands: vec![commands::register_commands()],
            listener: |ctx, event, framework, data| {
                Box::pin(events::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .client_settings(|s| s.intents(GatewayIntents::all()))
        .run()
        .await
        .unwrap();
}
