use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const STORAGE_FILE: &str = "chats.json";
const CHAT_STORAGE_FILE: &str = "chats.txt";
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ChatStorage {
    #[serde(default)]
    chats: HashMap<String, String>, // name -> run_id
    #[serde(default)]
    contents: HashMap<String, String>,
}

impl ChatStorage {
    pub fn load() -> Result<Self> {
        if !Path::new(STORAGE_FILE).exists() {
            return Ok(Self::default());
        }
       let data = fs::read_to_string(STORAGE_FILE)
            .with_context(|| format!("Failed to read storage file: {}", STORAGE_FILE))?;
        
        serde_json::from_str(&data)
            .with_context(|| format!("Failed to parse storage file: {}", STORAGE_FILE))
    }

    fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialize chat storage")?;
            
        fs::write(STORAGE_FILE, data)
            .with_context(|| format!("Failed to write to storage file: {}", STORAGE_FILE))
    }

    pub fn add_chat(&mut self, name: &str, run_id: &str) -> Result<()> {
        if self.chats.contains_key(name) {
            anyhow::bail!("A chat with name '{}' already exists", name);
        }
        
        self.chats.insert(name.to_string(), run_id.to_string());
        self.save()
    }

    pub fn get_run_id(&self, name: &str) -> Result<String> {
        self.chats.get(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No chat found with name '{}'", name))
    }

    pub fn list_chats(&self) -> Vec<&String> {
        self.chats.keys().collect()
    }

    pub fn stor_chat_history(&self, id: &str, chat_text: &str) -> Result<()> {
       fs::write(CHAT_STORAGE_FILE, chat_text).with_context(|| format!("faild t owrite to {}", CHAT_STORAGE_FILE));
       Ok(())

    }
}
