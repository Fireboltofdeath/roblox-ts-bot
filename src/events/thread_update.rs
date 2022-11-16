use std::{future::IntoFuture, sync::Arc};

use tokio::sync::Mutex;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::ThreadUpdate;

use crate::config::BotConfig;

pub async fn handle(event: ThreadUpdate, config: Arc<Mutex<BotConfig>>, http: Arc<Client>) {
    let config = config.lock().await;
    if config.help_channel_id() != event.parent_id {
        return;
    }

    let unsolved_tag_id = match config.unsolved_tag_id() {
        Some(value) => value,
        None => return,
    };

    let solved_tag_id = match config.solved_tag_id() {
        Some(value) => value,
        None => return,
    };

    let mut applied_tags = event.applied_tags.clone().unwrap_or_default();
    let has_solved = applied_tags.iter().any(|&v| v == solved_tag_id);
    let has_unsolved = applied_tags.iter().any(|&v| v == unsolved_tag_id);

    if has_solved && has_unsolved {
        applied_tags.retain(|&v| v != unsolved_tag_id);
    } else if !has_solved && !has_unsolved {
        applied_tags.push(unsolved_tag_id);
    } else {
        return;
    }

    http.update_thread(event.id)
        .applied_tags(Some(&applied_tags))
        .into_future()
        .await
        .ok();
}
