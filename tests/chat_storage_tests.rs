use anyhow::Result;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

// Import the storage module from the main crate
use web_term::storage::ChatStorage;

#[test]
fn test_new_chat_saves_initial_conversation() -> Result<()> {
    // Create a temporary directory for our test
    let test_dir = tempdir()?;
    let test_path = test_dir.path();
    
    // Create a new ChatStorage instance
    let mut storage = ChatStorage::new_with_path(test_path)?;
    
    // Test data
    let chat_name = "test_chat";
    let run_id = "test_run_id";
    let user_message = "Hello, this is a test message";
    let ai_response = "Hello! I'm an AI assistant. How can I help you today?";
    
    // Add a new chat
    storage.add_chat(chat_name, run_id)?;
    
    // Store the initial conversation
    storage.store_chat_history(chat_name, user_message, ai_response)?;
    
    // Check if the file exists
    let chat_file_path = test_path.join(format!("{}.txt", chat_name));
    assert!(chat_file_path.exists(), "Chat file should exist");
    
    // Read the file content
    let content = fs::read_to_string(&chat_file_path)?;
    
    // Verify content format
    assert!(content.contains("User: Hello, this is a test message"), 
            "File should contain user message with 'User:' prefix");
    assert!(content.contains("AI: Hello! I'm an AI assistant. How can I help you today?"), 
            "File should contain AI response with 'AI:' prefix");
    
    Ok(())
}

#[test]
fn test_continue_chat_appends_to_history() -> Result<()> {
    // Create a temporary directory for our test
    let test_dir = tempdir()?;
    let test_path = test_dir.path();
    
    // Create a new ChatStorage instance
    let mut storage = ChatStorage::new_with_path(test_path)?;
    
    // Test data
    let chat_name = "test_chat";
    let run_id = "test_run_id";
    let initial_user_message = "What's the weather like?";
    let initial_ai_response = "I don't have access to real-time weather data.";
    
    // Add a new chat and store initial conversation
    storage.add_chat(chat_name, run_id)?;
    storage.store_chat_history(chat_name, initial_user_message, initial_ai_response)?;
    
    // Continue the conversation
    let follow_up_user_message = "How about tomorrow's forecast?";
    let follow_up_ai_response = "I still don't have access to weather forecasts.";
    
    // Store the continued conversation
    storage.store_chat_history(chat_name, follow_up_user_message, follow_up_ai_response)?;
    
    // Check file content
    let chat_file_path = test_path.join(format!("{}.txt", chat_name));
    let content = fs::read_to_string(&chat_file_path)?;
    
    // Verify all messages are present
    assert!(content.contains("User: What's the weather like?"), 
            "File should contain initial user message");
    assert!(content.contains("AI: I don't have access to real-time weather data."), 
            "File should contain initial AI response");
    assert!(content.contains("User: How about tomorrow's forecast?"), 
            "File should contain follow-up user message");
    assert!(content.contains("AI: I still don't have access to weather forecasts."), 
            "File should contain follow-up AI response");
    
    // Ensure messages are in the correct order
    let user1_pos = content.find("User: What's the weather like?").unwrap();
    let ai1_pos = content.find("AI: I don't have access to real-time weather data.").unwrap();
    let user2_pos = content.find("User: How about tomorrow's forecast?").unwrap();
    let ai2_pos = content.find("AI: I still don't have access to weather forecasts.").unwrap();
    
    assert!(user1_pos < ai1_pos, "Initial user message should come before initial AI response");
    assert!(ai1_pos < user2_pos, "Initial AI response should come before follow-up user message");
    assert!(user2_pos < ai2_pos, "Follow-up user message should come before follow-up AI response");
    
    Ok(())
}

#[test]
fn test_multiple_chats_separate_histories() -> Result<()> {
    // Create a temporary directory for our test
    let test_dir = tempdir()?;
    let test_path = test_dir.path();
    
    // Create a new ChatStorage instance
    let mut storage = ChatStorage::new_with_path(test_path)?;
    
    // Test data for first chat
    let chat1_name = "chat1";
    let run1_id = "run_id_1";
    let user1_message = "Tell me about Rust";
    let ai1_response = "Rust is a systems programming language...";
    
    // Test data for second chat
    let chat2_name = "chat2";
    let run2_id = "run_id_2";
    let user2_message = "Tell me about Python";
    let ai2_response = "Python is a high-level programming language...";
    
    // Add and store first chat
    storage.add_chat(chat1_name, run1_id)?;
    storage.store_chat_history(chat1_name, user1_message, ai1_response)?;
    
    // Add and store second chat
    storage.add_chat(chat2_name, run2_id)?;
    storage.store_chat_history(chat2_name, user2_message, ai2_response)?;
    
    // Check first chat file
    let chat1_file_path = test_path.join(format!("{}.txt", chat1_name));
    let content1 = fs::read_to_string(&chat1_file_path)?;
    
    assert!(content1.contains("User: Tell me about Rust"), 
            "First chat file should contain first user message");
    assert!(content1.contains("AI: Rust is a systems programming language..."), 
            "First chat file should contain first AI response");
    assert!(!content1.contains("Tell me about Python"), 
            "First chat file should not contain second user message");
    
    // Check second chat file
    let chat2_file_path = test_path.join(format!("{}.txt", chat2_name));
    let content2 = fs::read_to_string(&chat2_file_path)?;
    
    assert!(content2.contains("User: Tell me about Python"), 
            "Second chat file should contain second user message");
    assert!(content2.contains("AI: Python is a high-level programming language..."), 
            "Second chat file should contain second AI response");
    assert!(!content2.contains("Tell me about Rust"), 
            "Second chat file should not contain first user message");
    
    Ok(())
}

#[test]
fn test_error_handling_for_nonexistent_chat() -> Result<()> {
    // Create a temporary directory for our test
    let test_dir = tempdir()?;
    let test_path = test_dir.path();
    
    // Create a new ChatStorage instance
    let storage = ChatStorage::new_with_path(test_path)?;
    
    // Try to store history for a chat that doesn't exist
    let result = storage.store_chat_history(
        "nonexistent_chat", 
        "This chat doesn't exist", 
        "This should fail"
    );
    
    // Verify that an error was returned
    assert!(result.is_err(), "Storing history for nonexistent chat should fail");
    
    Ok(())
}

#[test]
fn test_chat_history_persists_after_reload() -> Result<()> {
    // Create a temporary directory for our test
    let test_dir = tempdir()?;
    let test_path = test_dir.path();
    
    // Create and use a ChatStorage instance
    {
        let mut storage = ChatStorage::new_with_path(test_path)?;
        
        let chat_name = "persistent_chat";
        let run_id = "persistent_run_id";
        let user_message = "Will this be saved?";
        let ai_response = "Yes, it will be saved!";
        
        storage.add_chat(chat_name, run_id)?;
        storage.store_chat_history(chat_name, user_message, ai_response)?;
        
        // Storage is dropped here, simulating program exit
    }
    
    // Create a new ChatStorage instance, simulating program restart
    {
        let mut storage = ChatStorage::new_with_path(test_path)?;
        
        // Continue the conversation
        let chat_name = "persistent_chat";
        let follow_up_message = "What about this message?";
        let follow_up_response = "This will also be saved!";
        
        storage.store_chat_history(chat_name, follow_up_message, follow_up_response)?;
        
        // Check file content
        let chat_file_path = test_path.join(format!("{}.txt", chat_name));
        let content = fs::read_to_string(&chat_file_path)?;
        
        // Verify all messages are present
        assert!(content.contains("User: Will this be saved?"), 
                "File should contain initial user message after reload");
        assert!(content.contains("AI: Yes, it will be saved!"), 
                "File should contain initial AI response after reload");
        assert!(content.contains("User: What about this message?"), 
                "File should contain follow-up user message");
        assert!(content.contains("AI: This will also be saved!"), 
                "File should contain follow-up AI response");
    }
    
    Ok(())
}
