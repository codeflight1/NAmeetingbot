mod commands;
mod types;

use crate::types::meeting::{Meeting, RepeatType};
use chrono::Utc;
use dotenv;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use std::env;
use std::sync::Mutex;

struct Handler {
    meetings: Mutex<Vec<Meeting>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "meeting" => commands::meetings::run(&command.data.options),
                "list" => commands::list::run(&command.data.options, &self.meetings),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::meetings::register(command))
                .create_application_command(|command| commands::list::register(command))
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let meeting = Meeting {
        name: "Test".to_string(),
        first_date: Utc::now(),
        next_date: Utc::now(),
        repeat: RepeatType::Weekly,
    };

    let meeting1 = Meeting {
        repeat: RepeatType::BiWeekly,
        ..meeting.clone()
    };

    let meeting2 = Meeting {
        repeat: RepeatType::Monthly,
        ..meeting.clone()
    };

    let meeting3 = Meeting {
        repeat: RepeatType::None,
        ..meeting.clone()
    };

    let handler = Handler {
        meetings: Mutex::new(vec![
            meeting.clone(),
            meeting1.clone(),
            meeting2.clone(),
            meeting3.clone(),
        ]),
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
