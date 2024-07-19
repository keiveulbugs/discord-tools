/// `say_reply()` but then ephemeral.
pub async fn say_reply_ephemeral(
    ctx: crate::Context<'_>,
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
