use std::sync::Arc;

use tokio::sync::Mutex;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::config::BotConfig;

pub async fn handle(event: MessageCreate, config: Arc<Mutex<BotConfig>>) {
    if event.author.id != BotConfig::owner() {
        return;
    }

    let mut config = config.lock().await;
    let mut message = event.content.split_whitespace();
    if message.next() == Some("!configureforum") {
        let help_channel_id = message.next();
        let unsolved_tag_id = message.next();
        let solved_tag_id = message.next();

        config.help_channel_id = help_channel_id.map(ToString::to_string);
        config.unsolved_tag_id = unsolved_tag_id.map(ToString::to_string);
        config.solved_tag_id = solved_tag_id.map(ToString::to_string);
        config.save();
    };
}
