use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

const STORAGE_FILE: &str = "chats.json";
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ChatStorage {
    #[serde(default)]
    chats: HashMap<String, String>, // name -> run_id
    #[serde(default)]
    contents: HashMap<String, String>,
    #[serde(skip)]
    base_path: PathBuf,
}

impl ChatStorage {
    pub fn load() -> Result<Self> {
        Self::new_with_path(Path::new("."))
    }
    
    pub fn new_with_path(base_path: &Path) -> Result<Self> {
        let storage_path = base_path.join(STORAGE_FILE);
        
        if !storage_path.exists() {
            let mut storage = Self::default();
            storage.base_path = base_path.to_path_buf();
            return Ok(storage);
        }
        
        let data = fs::read_to_string(&storage_path)
            .with_context(|| format!("Failed to read storage file: {}", storage_path.display()))?;
        
        let mut storage: Self = serde_json::from_str(&data)
            .with_context(|| format!("Failed to parse storage file: {}", storage_path.display()))?;
        
        storage.base_path = base_path.to_path_buf();
        Ok(storage)
    }

    fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialize chat storage")?;
        
        let storage_path = self.base_path.join(STORAGE_FILE);
        fs::write(&storage_path, data)
            .with_context(|| format!("Failed to write to storage file: {}", storage_path.display()))
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

    pub fn store_chat_history(&self, chat_name: &str, user_message: &str, ai_response: &str) -> Result<()> {
        // Check if the chat exists
        if !self.chats.contains_key(chat_name) {
            return Err(anyhow::anyhow!("No chat found with name '{}'", chat_name));
        }
        
        let chat_file_path = self.base_path.join(format!("{}.txt", chat_name));
        
        // Format the messages
        let formatted_user_message = format!("User: {}\n", user_message);
        let formatted_ai_response = format!("AI: {}\n\n", ai_response);
        
        // Append to file if it exists, create if it doesn't
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&chat_file_path)
            .with_context(|| format!("Failed to open chat file: {}", chat_file_path.display()))?;
        
        // Write the messages
        file.write_all(formatted_user_message.as_bytes())
            .with_context(|| format!("Failed to write user message to file: {}", chat_file_path.display()))?;
        
        file.write_all(formatted_ai_response.as_bytes())
            .with_context(|| format!("Failed to write AI response to file: {}", chat_file_path.display()))?;
        
        Ok(())
    }
    
    // For backward compatibility
    pub fn stor_chat_history(&self, _id: &str, chat_text: &str) -> Result<()> {
        // This is kept for backward compatibility
        // In the new version, we should use store_chat_history instead
        let chat_file_path = self.base_path.join("chats.txt");
        fs::write(&chat_file_path, chat_text)
            .with_context(|| format!("Failed to write to {}", chat_file_path.display()))?;
        Ok(())
    }
}
