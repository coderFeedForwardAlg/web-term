use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const TOOLHOUSE_BASE_URL: &str = "https://agents.toolhouse.ai/1356d033-69af-4a2e-9da2-1c1ee3807902";

#[derive(Debug, Serialize)]
struct ChatMessage {
    message: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    response: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

async fn start_new_chat(message: &str) -> Result<String> {
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
    
    Ok(run_id)
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
        /// Your message to start the chat
        message: String,
    },
    /// Continue an existing chat session
    Continue {
        /// The run ID of the existing chat
        run_id: String,
        /// Your message
        message: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::New { message } => {
            println!("Starting new chat...");
            let run_id = start_new_chat(&message).await?;
            println!("New chat started with ID: {}", run_id);
        }
        Commands::Continue { run_id, message } => {
            println!("Continuing chat {}...", run_id);
            continue_chat(&run_id, &message).await?;
        }
    }

    Ok(())
}
