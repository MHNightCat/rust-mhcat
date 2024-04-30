use serenity::all::{Command, Context};
use serenity::model::gateway::Ready;

use crate::commands;

pub async fn bot_ready(ctx: Context, ready: Ready) {
    if let Some(shard) = ready.shard {
        println!(
            "{} is connected on shard {}/{}! guilds: {}",
            ready.user.name,
            shard.id,
            shard.total,
            ready.guilds.len()
        );
    }

    let global_commands = Command::set_global_commands(
        &ctx.http,
        vec![
            commands::ping::register(),
            commands::locales::register(),
        ],
    )
    .await;
    println!("I created the following global slash command: {global_commands:#?}");
}
