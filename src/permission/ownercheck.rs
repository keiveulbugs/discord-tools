pub async fn ownercheck<U, E>(ctx: poise::Context<'_, U, E>) -> bool {
    ctx.framework().options.owners.contains(&ctx.author().id)
}
