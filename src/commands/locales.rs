use rust_i18n::t;
use serenity::all::{CommandOptionType, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Hey, I'm alive!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("set-language")
        .name_localized("en-US", t!("slash_command.locales.name", locale = "en"))
        .description("Set MHCAT language")
        .description_localized(
            "en-US",
            t!("slash_command.locales.description", locale = "en"),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "language",
                "Language to change to",
            )
            .name("language")
            // name localized
            .name_localized(
                "en-US",
                t!("slash_command.locales.options.language.name", locale = "en"),
            )
            .description("Select a language")
            .description_localized(
                "en-US",
                t!(
                    "slash_command.locales.options.language.description",
                    locale = "en"
                ),
            )
            .required(true),
        )
}
