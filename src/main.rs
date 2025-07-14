mod storage;

use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use storage::ChatStorage;
use std::fs;

const TOOLHOUSE_BASE_URL: &str = "";

#[derive(Debug, Serialize)]
struct ChatMessage {
    message: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    response: Option<String>,
}

async fn start_new_chat(message: &str) -> Result<(String, String)> {
    let client = reqwest::Client::new();
    
    println!("Sending request to: {}", TOOLHOUSE_BASE_URL);
    let response = client
        .post(TOOLHOUSE_BASE_URL)
        .json(&ChatMessage {
            message: message.to_string(),
        })
        .send()
        .await?;
    
    // Save headers before consuming the response
    let response_headers = response.headers().clone();
    
    // Print response status and headers for debugging
    println!("Response status: {}", response.status());
    println!("Response headers: {:?}", response_headers);
    
    // Read the response text for debugging
    let response_text = response.text().await?;
    println!("Raw response: {}", response_text);
    
    // Try to parse as JSON if not empty
    if !response_text.trim().is_empty() {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&response_text) {
            println!("Parsed JSON response: {}", parsed);
        }
    }
    
    // Try to get the run ID from headers
    let run_id = response_headers
        .get("x-toolhouse-run-id")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!(
            "No x-toolhouse-run-id in response headers. Full response: {}", 
            response_text
        ))?;
    
    Ok((run_id, response_text))
}

async fn continue_chat(run_id: &str, message: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}", TOOLHOUSE_BASE_URL, run_id);
    
    println!("Sending request to: {}", url);
    let response = client
        .put(&url)
        .json(&ChatMessage {
            message: message.to_string(),
        })
        .send()
        .await?;
    
    // Save headers before consuming the response
    let response_headers = response.headers().clone();
    
    // Print response status and headers for debugging
    println!("Response status: {}", response.status());
    println!("Response headers: {:?}", response_headers);
    
    // Read the response text for debugging
    let response_text = response.text().await?;
    println!("Raw response: {}", response_text);
    
    // Try to parse as JSON if not empty
    if !response_text.trim().is_empty() {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&response_text) {
            println!("Parsed JSON response: {}", parsed);
        }
        
        // Try to parse as ChatResponse
        if let Ok(response_body) = serde_json::from_str::<ChatResponse>(&response_text) {
            if let Some(response_text) = response_body.response {
                println!("Response: {}", response_text);
            }
        }
    }
    
    Ok(())
}

#[derive(Debug, Parser)]
#[command(name = "toolhouse-chat")]
#[command(about = "CLI for interacting with Toolhouse AI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start a new chat session
    New {
        /// Name for this chat
        name: String,
        /// Your message to start the chat
        message: String,
    },
    /// Continue an existing chat session
    Continue {
        /// The name of the chat to continue
        name: String,
        /// Your message
        message: String,
    },
    /// List all available chats
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let mut storage = ChatStorage::load()?;

    match args.command {
        Commands::New { name, message } => {
            println!("Starting new chat '{}'...", name);
            let (run_id, ai_message) = start_new_chat(&message).await?;
            storage.add_chat(&name, &run_id)?;
            println!("New chat '{}' started with ID: {}", name, run_id);
            let file_name = format!("{}.txt", name);
            let _ = fs::write(file_name.clone(), message)?;
            let _ = fs::write(file_name.clone(), ai_message)?;
        }
        Commands::Continue { name, message } => {
            println!("Continuing chat '{}'...", name);
            let run_id = storage.get_run_id(&name)?;
            continue_chat(&run_id, &message).await?;
        }
        Commands::List => {
            let chats = storage.list_chats();
            if chats.is_empty() {
                println!("No chats found.");
            } else {
                println!("Available chats:");
                for chat in chats {
                    println!("- {}", chat);
                }
            }
        }
    }

    Ok(())
}
