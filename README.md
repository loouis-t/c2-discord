# C2 Client

This is a C2 client allowing to download files and execute bash 
commands on a distant machine, using a discord bot.

## Installation

### Configuration

- Go to [Discord Developer Portal](https://discord.com/developers/applications),
create a new application, and a bot. Copy the token and paste it in the
`.env` file.
- Go to the `OAuth2` tab, select the `bot` scope, and copy the generated URL
to invite the bot to your server.
- Make sure your bot has the `Send Messages` and `Read Message History` permissions.
- Make sure all "Privileged Gateway Intents" are enabled in the bot settings
(in developer portal).

### Build

```bash
cargo build --release
```

### Execution

```bash
cargo run --release
```

## Usage

The bot will listen to messages in the server, and execute the commands.

### Commands

- `!download <path>`: Download a file from the given path.
- `!run <command>`: Execute a bash command on the machine.
