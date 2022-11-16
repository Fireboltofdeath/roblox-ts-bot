use std::sync::Arc;

use anyhow::Result;
use config::BotConfig;
use futures::StreamExt;
use tokio::sync::Mutex;
use twilight_gateway::{Intents, Shard};
use twilight_http::Client;

use crate::events::process_event;

mod config;
mod events;

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let client = Arc::new(Client::new(token.to_string()));
    let config = Arc::new(Mutex::new(BotConfig::get_saved()));
    let (shard, mut events) = Shard::new(
        token.to_string(),
        Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
    );

    shard.start().await?;

    while let Some(event) = events.next().await {
        tokio::spawn(process_event(event, config.clone(), client.clone()));
    }

    Ok(())
}
