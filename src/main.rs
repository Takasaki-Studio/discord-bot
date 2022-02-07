mod commands;
mod config;

use config::Config;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};

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
            ..Default::default()
        })
        .run()
        .await
        .unwrap();
}
