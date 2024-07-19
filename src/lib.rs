pub mod message;
pub mod permission;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
#[derive(Debug, Clone)]
pub struct Data {}
