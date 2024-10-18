use crate::lib::response::ResponseData;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub fn run(_command: &ApplicationCommandInteraction) -> ResponseData {
    ResponseData {
        command: "ping".to_string(),
        content: Some("Pong!".to_string()),
        embeds: None,
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Replies with Pong!")
}
