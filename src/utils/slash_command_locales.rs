use serenity::all::{CreateCommand, CreateCommandOption};

const LOCALES: [&str; 3] = ["en-US", "zh-TW", "zh-CN"];

pub fn slash_command_locales(
    command: CreateCommand,
    name: String,
    description: String,
) -> CreateCommand {
    let mut command = command;
    for locale in LOCALES {
        let command_name = t!(&name, locale = locale);
        let command_description = t!(&description, locale = locale);
        command = command.name_localized(locale, command_name);
        command = command.description_localized(locale, command_description);
    }
    command
}

pub fn slash_command_options_locales(
    command_option: CreateCommandOption,
    name: String,
    description: String
) -> CreateCommandOption {
    let mut command_option = command_option;
    for locale in LOCALES {
        let command_name = t!(&name, locale = locale);
        let command_description = t!(&description, locale = locale);
        command_option = command_option.name_localized(locale, command_name);
        command_option = command_option.description_localized(locale, command_description);
    }
    command_option
}
