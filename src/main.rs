mod commands;
mod config;

use config::Config;
use poise::{
    serenity_prelude::{self, ReactionType},
    Event, Framework, FrameworkOptions, PrefixFrameworkOptions,
};

pub struct Data {
    pub config: Config,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

async fn event_handler(
    ctx: &serenity_prelude::Context,
    event: &Event<'_>,
    _framework: &Framework<Data, Error>,
    states: &Data,
) -> Result<(), Error> {
    if let Event::Message { new_message } = event {
        if new_message.channel_id.0 == states.config.suggestion_channel {
            new_message
                .react(ctx, ReactionType::Unicode("👍".to_owned()))
                .await?;
            new_message
                .react(ctx, ReactionType::Unicode("👎".to_owned()))
                .await?;
        }
    }
    Ok(())
}

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
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .run()
        .await
        .unwrap();
}
