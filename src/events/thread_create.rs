use std::{future::IntoFuture, sync::Arc};

use tokio::sync::Mutex;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::ThreadCreate;

use crate::config::BotConfig;

pub async fn handle(event: ThreadCreate, config: Arc<Mutex<BotConfig>>, http: Arc<Client>) {
    let config = config.lock().await;
    if config.help_channel_id() != event.parent_id {
        return;
    }

    let unsolved_tag_id = match config.unsolved_tag_id() {
        Some(value) => value,
        None => return,
    };

    let mut applied_tags = event.applied_tags.clone().unwrap_or_default();
    applied_tags.push(unsolved_tag_id);

    http.update_thread(event.id)
        .applied_tags(Some(&applied_tags))
        .into_future()
        .await
        .ok();
}
