/// Reply a simple embed to the author.
pub async fn reply_simple_embed(
    ctx: crate::Context<'_>,
    title: impl Into<String>,
    description: impl Into<String>,
    ephemeral: bool,
) -> Result<poise::ReplyHandle<'_>, serenity::Error> {
    poise::send_reply(
        ctx,
        poise::CreateReply::default()
            .embed(
                serenity::all::CreateEmbed::default()
                    .title(title)
                    .description(description),
            )
            .ephemeral(ephemeral),
    )
    .await
}
