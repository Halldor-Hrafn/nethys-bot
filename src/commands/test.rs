use crate::lib::response::ResponseData;

use serenity::builder::CreateApplicationCommand;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub fn run(command: &ApplicationCommandInteraction) -> ResponseData {
    let options = &command.data.options;

    let content = options
        .iter()
        .find(|option| option.name == "content")
        .map_or(false, |option| option.value.as_ref().unwrap().as_bool().unwrap());
    let embed = options
        .iter()
        .find(|option| option.name == "embed")
        .map_or(false, |option| option.value.as_ref().unwrap().as_bool().unwrap());

    let mut response = ResponseData {
        command: "test".to_string(),
        content: None,
        embeds: None,
    };

    if content {
        response.content = Some("This is a test message".to_string());
    }

    if embed {
        let embed = create_embed();

        response.embeds = Some(vec![embed]);
    }

    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("test")
        .description("A command used to test various functions of the bot")
        .create_option(|option| {
            option
                .name("content")
                .description("The content to display")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("embed")
                .description("The embed to display")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
}

pub fn create_embed() -> CreateEmbed {
    CreateEmbed::default()
        .title("This is a test embed")
        .description("This is a test embed description")
        .field("Field 1", "Field 1 value", false)
        .to_owned()
}
