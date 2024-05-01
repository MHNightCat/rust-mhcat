use crate::utils::slash_command_locales;

use serenity::all::{CommandOptionType, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Hey, I'm alive!".to_string()
}

pub fn register() -> CreateCommand {
    let mut slash_command_option_locale = CreateCommandOption::new(
        CommandOptionType::String,
        "language",
        "Language to change to",
    )
    .name("language")
    // name localized
    .description("Select a language")
    .required(true);

    slash_command_option_locale = slash_command_locales::slash_command_options_locales(
        slash_command_option_locale,
        "slash_command.locales.options.language.name".to_string(),
        "slash_command.locales.options.language.description".to_string(),
    );

    let mut slash_command = CreateCommand::new("set-language")
        .description("Set MHCAT language")
        .add_option(slash_command_option_locale);

    slash_command = slash_command_locales::slash_command_locales(
        slash_command,
        "slash_command.locales.name".to_string(),
        "slash_command.locales.description".to_string(),
    );

    return slash_command;
}
