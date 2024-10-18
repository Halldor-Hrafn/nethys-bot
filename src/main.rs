pub mod commands;

extern crate dotenv;

mod lib;

use crate::lib::response::ResponseData;

use dotenv::dotenv;
use serenity::model::prelude::interaction;
use std::env;
use std::error::Error;

use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::id::GuildId;

async fn handle_command(command: &ApplicationCommandInteraction, ctx: &Context) -> Result<(), Box<dyn Error>> {
    let unknown_command_response: ResponseData = ResponseData {
        command: command.data.name.clone(),
        content: Some("Unknown command".to_string()),
        embeds: None,
    };

    let content = match command.data.name.as_str() {
        "ping" => commands::ping::run(command),

        "test" => commands::test::run(command),

        _ => unknown_command_response,
    };

    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(move |message| {
                    if content.content.is_some() {
                        message.content(content.content.unwrap());
                    } else {
                        // pass
                    }

                    if content.embeds.is_some() {
                        message.add_embeds(content.embeds.unwrap());
                    } else {
                        // pass
                    }

                    message
                })
        }).await {
            println!("Error sending response: {:?}", why);
        }

    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            handle_command(&command, &ctx).await;
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected a guild ID in the environment")
                .parse()
                .expect("Failed to parse GUILD_ID"),
        );

        // let _dev_commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        //     commands
        //         .create_application_command(|command| commands::ping::register(command))
        //         .create_application_command(|command| commands::test::register(command))
        // }).await;

        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = dotenv::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
