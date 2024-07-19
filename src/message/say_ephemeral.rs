pub async fn say_ephemeral<U, E>(
    ctx: poise::Context<'_, U, E>,
    text: impl Into<String>,
) -> Result<poise::ReplyHandle<'_>, serenity::Error> {
    poise::send_reply(
        ctx,
        poise::CreateReply::default()
            .content(text.into())
            .ephemeral(true),
    )
    .await
}
