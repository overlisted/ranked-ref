use std::fs::File;
use std::path::Path;
use std::sync::RwLock;

use dashmap::DashMap;
use serenity::model::id::GuildId;
use serenity::prelude::TypeMapKey;

use crate::system::Leaderboard;
use std::io::Write;

pub trait Backend {
    fn serialize(&mut self, lbs: &DashMap<GuildId, Leaderboard>);
    fn deserialize(&mut self) -> DashMap<GuildId, Leaderboard>;
}

pub struct JsonFileBackend {
    pub file: File,
}

impl JsonFileBackend {
    pub fn new(filename: &Path) -> JsonFileBackend {
        let file = if filename.exists() {
            File::open(filename).expect("Failed to open JSON")
        } else {
            let file = File::create(filename).expect("Failed to create JSON");
            let defaults: DashMap<(), ()> = DashMap::new();
            serde_json::to_writer(&file, &defaults).expect("Failed to initialize JSON");

            file
        };

        JsonFileBackend { file }
    }
}

impl Backend for JsonFileBackend {
    fn serialize(&mut self, lbs: &DashMap<GuildId, Leaderboard>) {
        serde_json::to_writer(&self.file, lbs).expect("Failed to serialize to JSON");
        self.file.flush().expect("Failed to serialize to JSON!");
    }

    fn deserialize(&mut self) -> DashMap<GuildId, Leaderboard> {
        serde_json::from_reader(&self.file).expect("Failed to deserialize from JSON")
    }
}

impl TypeMapKey for JsonFileBackend {
    type Value = RwLock<JsonFileBackend>;
}
