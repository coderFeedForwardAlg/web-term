mod storage;

use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use storage::ChatStorage;
use std::io::{self, Write};

const TOOLHOUSE_BASE_URL: &str = "https://agents.toolhouse.ai/1356d033-69af-4a2e-9da2-1c1ee3807902" ;

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

async fn continue_chat(run_id: &str, message: &str) -> Result<String> {
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
                return Ok(response_text);
            }
        }
    }
    
    // If we couldn't parse a proper response, return the raw text
    Ok(response_text)
}

async fn chat_loop(storage: &mut ChatStorage, name: &str, run_id: &str) -> Result<()> {
    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut user_message = String::new();
        io::stdin().read_line(&mut user_message)?;
        let user_message = user_message.trim();

        if user_message == "/bye" {
            break;
        }

        if user_message.is_empty() {
            continue;
        }

        let ai_response = continue_chat(run_id, user_message).await?;
        println!("AI: {}", ai_response);

        storage.store_chat_history(name, user_message, &ai_response)?;
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
    Chat {
        /// The name of the chat to continue
        name: String,
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
            println!("AI: {}", ai_message);
            // Store the conversation history with proper formatting
            storage.store_chat_history(&name, &message, &ai_message)?;
            chat_loop(&mut storage, &name, &run_id).await?;
        }
        Commands::Chat { name } => {
            println!("Continuing chat '{}'...", name);
            let run_id = storage.get_run_id(&name)?;
            chat_loop(&mut storage, &name, &run_id).await?;
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
