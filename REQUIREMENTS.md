# Conversation History Storage Requirements

## Overview
This document outlines the requirements for enhancing the conversation history storage functionality in the web-term application. Currently, only the first AI response is saved to a file, but the requirement is to store the complete conversation history.

## Current Behavior
- When a new chat is created, a text file is created with the name of the chat (e.g., `{name}.txt`).
- Only the initial user message and the first AI response are written to this file, overwriting each other.
- Subsequent messages in the conversation are not saved.
- When continuing a conversation with `--continue`, these messages are not appended to the file.

## Required Behavior
1. **Complete Conversation Storage**:
   - The text file should contain the entire conversation history.
   - This includes:
     - The initial user question/message
     - The AI's first response
     - All subsequent user messages
     - All subsequent AI responses
     - Any continued conversations initiated with the `--continue` command

2. **Formatting**:
   - Messages should be clearly delineated to distinguish between user and AI.
   - Each message should be prefixed with "User:" or "AI:" to identify the speaker.
   - Messages should be separated by a newline for readability.

3. **Persistence**:
   - The conversation history should be persistent across multiple sessions.
   - When a user continues a conversation, the new messages should be appended to the existing file.

4. **File Structure**:
   - Each chat should have its own dedicated file named `{chat_name}.txt`.
   - The file should be human-readable and easily accessible.

5. **Error Handling**:
   - The application should handle file I/O errors gracefully.
   - If a file cannot be read or written to, appropriate error messages should be displayed.

## Technical Implementation Considerations
- The `ChatStorage` struct should be enhanced to maintain the full conversation history.
- The `stor_chat_history` method should be updated to append new messages rather than overwrite.
- Both the `start_new_chat` and `continue_chat` functions should update the conversation history.
- The storage mechanism should be thread-safe and handle concurrent access if applicable.

## Success Criteria
- All conversation turns (user and AI) are properly saved to the appropriate file.
- The conversation history is maintained across multiple sessions.
- The file format is human-readable and clearly distinguishes between user and AI messages.
- The implementation passes all test cases.
