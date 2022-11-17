use std::{fs, io::ErrorKind};

use serde::{Deserialize, Serialize};
use twilight_model::id::{
    marker::{ChannelMarker, TagMarker, UserMarker},
    Id,
};

#[derive(Default, Serialize, Deserialize)]
pub struct BotConfig {
    pub help_channel_id: Option<String>,
    pub unsolved_tag_id: Option<String>,
    pub solved_tag_id: Option<String>,
}

impl BotConfig {
    pub fn owner() -> Id<UserMarker> {
        let id = std::env::var("OWNER").unwrap_or_else(|_| "259237045593964554".to_string());
        Id::new(id.parse::<u64>().unwrap())
    }

    pub fn path() -> String {
        std::env::var("CONFIG_PATH").unwrap_or_else(|_| "./config.json".to_string())
    }

    pub fn get_saved() -> BotConfig {
        match fs::read(BotConfig::path()) {
            Ok(value) => serde_json::from_str(&String::from_utf8(value).unwrap()).unwrap(),
            Err(error) if matches!(error.kind(), ErrorKind::NotFound) => BotConfig::default(),
            Err(error) => panic!("Config could not be loaded {error:?}"),
        }
    }

    pub fn save(&self) {
        fs::write(BotConfig::path(), serde_json::to_string(self).unwrap())
            .expect("Config could not be saved");
    }

    pub fn help_channel_id(&self) -> Option<Id<ChannelMarker>> {
        self.help_channel_id
            .as_ref()?
            .parse::<u64>()
            .ok()
            .map(Id::new)
    }

    pub fn unsolved_tag_id(&self) -> Option<Id<TagMarker>> {
        self.unsolved_tag_id
            .as_ref()?
            .parse::<u64>()
            .ok()
            .map(Id::new)
    }

    pub fn solved_tag_id(&self) -> Option<Id<TagMarker>> {
        self.solved_tag_id
            .as_ref()?
            .parse::<u64>()
            .ok()
            .map(Id::new)
    }
}
