use crate::commands;

use serenity::all::{CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::prelude::*;

pub async fn slash_commands(ctx: Context, command: CommandInteraction) {
        println!("Received command interaction: {command:#?}");

        let content = match command.data.name.as_str() {
            "ping" => Some(commands::ping::run(&command.data.options())),
            "set-language" => Some(commands::locales::run(&command.data.options())),
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

