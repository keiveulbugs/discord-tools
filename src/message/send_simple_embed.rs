/// Send a simple embed.
pub async fn reply_simple_embed<U, E>(
    ctx: poise::Context<'_, U, E>,
    channelid: serenity::all::ChannelId,
    title: impl Into<String>,
    description: impl Into<String>,
) -> Result<serenity::all::Message, serenity::Error> {
    let message = channelid
        .send_message(
            ctx.serenity_context(),
            serenity::all::CreateMessage::default().embed(
                serenity::all::CreateEmbed::default()
                    .title(title)
                    .description(description),
            ),
        )
        .await?;
    Ok(message)
}
