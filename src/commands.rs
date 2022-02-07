use poise::{builtins, command};

use crate::Context;

type CommandResult = Result<(), crate::Error>;

#[command(prefix_command, hide_in_help)]
pub async fn register_commands(ctx: Context<'_>, #[flag] global: bool) -> CommandResult {
    register_commands_handle(ctx, global).await
}

async fn register_commands_handle(ctx: Context<'_>, global: bool) -> CommandResult {
    let owner = ctx.data().config.owner_id;
    let author = ctx.author().id.0;

    if owner == author {
        builtins::register_application_commands(ctx, global).await?;
    } else if let Context::Prefix(p_ctx) = ctx {
        p_ctx
            .msg
            .reply(
                ctx.discord(),
                "<:cross:940015680529526844> **|** Você não é dono do bot para realizar essa ação",
            )
            .await?;
    }

    Ok(())
}
