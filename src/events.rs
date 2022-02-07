use poise::{
    serenity_prelude::{
        self, Channel, Color, CreateEmbed, Member, Mentionable, ReactionType, User,
    },
    Event, Framework,
};

use crate::{helpers::AvatarOrDefault, Data, Error};

pub async fn event_handler(
    ctx: &serenity_prelude::Context,
    event: &Event<'_>,
    _framework: &Framework<Data, Error>,
    states: &Data,
) -> Result<(), Error> {
    fn welcome_embed<'a>(
        embed: &'a mut CreateEmbed,
        member: &Option<Member>,
        user: &User,
    ) -> &'a mut CreateEmbed {
        let modified = if let Some(member) = member {
            embed.field("Usuário:", member.mention(), true)
        } else {
            embed
        };

        modified
            .field(
                "Data de criação da conta:",
                format!("<t:{}:R>", user.created_at().unix_timestamp()),
                true,
            )
            .thumbnail(user.get_avatar_or_default())
            .footer(|f| f.text(user.id))
    }

    match event {
        Event::Message { new_message } => {
            if new_message.channel_id.0 == states.config.suggestion_channel {
                new_message
                    .react(ctx, ReactionType::Unicode("👍".to_owned()))
                    .await?;
                new_message
                    .react(ctx, ReactionType::Unicode("👎".to_owned()))
                    .await?;
            }
        }
        Event::GuildMemberAddition { new_member } => {
            let channel = ctx.http.get_channel(states.config.welcome_channel).await?;
            if let Channel::Guild(channel) = channel {
                channel
                    .send_message(ctx, |m| {
                        m.add_embed(|embed| {
                            welcome_embed(embed, &Some(new_member.clone()), &new_member.user)
                                .color(Color::DARK_GREEN)
                                .title(format!("{} entrou no servidor!", new_member.user.tag()))
                        })
                    })
                    .await?;
            }
        }
        Event::GuildMemberRemoval {
            member_data_if_available,
            user,
            ..
        } => {
            let channel = ctx.http.get_channel(states.config.welcome_channel).await?;
            if let Channel::Guild(channel) = channel {
                channel
                    .send_message(ctx, |m| {
                        m.add_embed(|embed| {
                            welcome_embed(embed, member_data_if_available, user)
                                .color(Color::RED)
                                .title(format!("{} saiu no servidor!", user.tag()))
                        })
                    })
                    .await?;
            }
        }
        _ => {}
    }
    Ok(())
}
