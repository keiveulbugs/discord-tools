/// Check if the invoker of the context is an admin of that guild.
pub async fn admincheck<U, E>(ctx: crate::Context<'_>) -> bool {
    ctx.author_member()
        .await
        .and_then(|m| m.permissions)
        .is_some_and(poise::serenity_prelude::Permissions::administrator)
}
