mod commands;
mod handler;
mod utils;

use serde::Deserialize;
use std::fs;
use std::process::exit;
use toml;

use serenity::async_trait;
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;
#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");


#[derive(Deserialize)]
struct Config {
    discord_token: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            handler::interaction::slash_commands(ctx, command).await;
        }
    }
    
    async fn ready(&self, ctx: Context, ready: Ready) {
        handler::bot_ready::bot_ready(ctx, ready).await;
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
