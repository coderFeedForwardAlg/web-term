# Web-Term

A command-line interface for interacting with Toolhouse AI chat sessions. This tool allows you to start new chat sessions, continue existing ones, and list all available chats.

## Installation

1. Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/)
2. Clone this repository
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The binary will be available at `target/release/web-term`

## Usage

### Start a New Chat

To start a new chat session, use the `new` command:

```bash
./web-term new --name "chat_name" --message "Your initial message here"
```

**Flags:**
- `--name` or `-n`: A name for your chat session (required)
- `--message` or `-m`: Your initial message to start the chat (required)

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
   ./web-term -- new "image_help" "How can I resize images using Python?"
   ```

2. Continue the conversation later:
   ```bash
   ./web-term -- continue "image_help" "That's helpful! How about batch processing?"
   ```

3. List all your chats:
   ```bash
   ./web-term list
   ```

## Notes

- Chat names must be unique
- The tool will automatically create and manage the necessary storage files
- All communication with the Toolhouse AI server is done over HTTPS
