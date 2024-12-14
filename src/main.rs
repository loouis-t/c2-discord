use dotenv::dotenv;
use std::env;
use std::process::Command;
use serenity::all::{CreateAttachment, CreateMessage, Ready};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match parse(&msg.content) {
            Ok((prefix, cmd)) => {
                match prefix.as_str() {
                    "!run" => answer(&ctx, &msg, bash(&cmd)).await,
                    "!download" => send_file(&ctx, &msg, &cmd).await,
                    _ => answer(&ctx, &msg, "Unknown command".to_string()).await,
                }
            },
            Err(_) => {},
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // load .env file into environment variables

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

/**
 * Parse the command and return a Result containing the prefix and the rest of the command.
 */
fn parse(cmd: &str) -> Result<(String, String), String> {
    let mut parts = cmd.trim().splitn(2, ' ');

    let prefix = parts.next().ok_or("Command is empty")?;
    if !prefix.starts_with('!') {
        return Err("No valid prefix found (must start with '!')".to_string());
    }

    let rest = parts.next().unwrap_or("").trim().to_string();
    Ok((prefix.to_string(), rest))
}

/**
 * Send a message to the channel
 */
async fn answer(ctx: &Context, msg: &Message, answer: String) {
    match msg.channel_id.say(&ctx.http, answer).await {
        Ok(_) => {},
        Err(why) => println!("Error sending message: {why:?}"),
    }
}

/**
 * Send a message to the channel with a file attached
 */
async fn send_file(ctx: &Context, msg: &Message, file_path: &str) {
    let builder = CreateMessage::new().content(
        format!("Here is the file `{}`: ", file_path)
    );
    let paths = [
        CreateAttachment::path(file_path).await.expect("Failed to create attachment")
    ];
    if let Err(why) = msg.channel_id.send_files(&ctx.http, paths, builder).await {
        println!("Error sending message: {why:?}");
    }
}

/**
 * Execute a bash command and return the output
 */
fn bash(cmd: &str) -> String {
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!(" {}", cmd))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        format!("Output: ```bash\n{}\n```", String::from_utf8_lossy(&output.stdout))
    } else {
        format!("Error: ```bash\n{}\n```", String::from_utf8_lossy(&output.stderr))
    }
}


