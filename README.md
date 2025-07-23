# Web-Term

A command-line interface for interacting with Toolhouse AI chat sessions. This tool allows you to start new chat sessions, continue existing ones, and list all available chats.

## Usage

### Start a New Chat

To start a new chat session, use the `new` command:

```bash
cargo run -- new "chat_name" "Your initial message here"
```


### Continue an Existing Chat

To continue an existing chat session:


### List All Chats

To see all your saved chat sessions:

```bash
cargo run -- list
```

## How It Works

- Each chat session is saved with a unique name and run ID
- Chat history is stored in a local JSON file (`chats.json`)
- The tool communicates with the Toolhouse AI API to send and receive messages

## Examples

1. Start a new chat about image processing:
   ```bash
   cargo run -- new "image_help" "How can I resize images using Python?"
   ```

2. Continue the conversation later:
   ```bash
   cargo run -- continue "image_help" "That's helpful! How about batch processing?"
   ```

## Notes

- Chat names must be unique
- The tool will automatically create and manage the necessary storage files
- All communication with the Toolhouse AI server is done over HTTPS
