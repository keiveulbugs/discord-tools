pub async fn ownercheck<U, E>(ctx: crate::Context<'_>) -> bool {
    ctx.framework().options.owners.contains(&ctx.author().id)
}
