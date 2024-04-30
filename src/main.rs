mod commands;

use serde::Deserialize;
use std::fs;
use std::process::exit;
use toml;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[derive(Deserialize)]
struct Config {
    discord_token: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            println!(
                "{} is connected on shard {}/{}! guilds: {}",
                ready.user.name,
                shard.id,
                shard.total,
                ready.guilds.len()
            );
        }

        let _ = Command::set_global_commands(
            &ctx.http,
            vec![
                commands::wonderful_command::register(),
                commands::ping::register(),
            ],
        )
        .await;
    }
}

#[tokio::main]
async fn main() {
    let config = load_config();

    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&config.discord_token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_shards(2).await {
        println!("Client error: {why:?}");
    }
}

fn load_config() -> Config {
    let filename = "config/config.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };

    return config;
}
