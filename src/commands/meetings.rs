use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::types::meeting::{Meeting, RepeatType};
use chrono::{DateTime, Utc};

pub fn run(options: &[CommandDataOption]) -> String {
    let mut meeting = Meeting {
        name: "placeholder".to_string(),
        first_date: Utc::now(),
        next_date: Utc::now(),
        repeat: RepeatType::None,
    };

    let nameobj = options
        .get(0)
        .expect("Expected name option")
        .resolved
        .as_ref()
        .expect("Expected datetime object");

    let datetimeobj = options
        .get(1)
        .expect("Expected datetime option")
        .resolved
        .as_ref()
        .expect("Expected datetime object");

    let repeatobj = options
        .get(2)
        .expect("Expected repeat option")
        .resolved
        .as_ref()
        .expect("Expected repeat object");

    if let CommandDataOptionValue::String(value) = nameobj {
        meeting.name = value.to_string();
    } else {
        return "Name parsing error".to_string();
    }

    if let CommandDataOptionValue::String(value) = datetimeobj {
        match DateTime::parse_from_str(
            (value.to_owned() + " +0000").as_str(),
            "%Y-%m-%d %H:%M:%S %z",
        ) {
            Ok(datetime) => {
                meeting.first_date = DateTime::<Utc>::from(datetime);
                meeting.next_date = meeting.first_date;
            }
            Err(err) => {
                return "Datetime parsing error: ".to_string() + &err.to_string();
            }
        };
    } else {
        return "Datetime parsing error".to_string();
    }

    if let CommandDataOptionValue::String(value) = repeatobj {
        match value.to_lowercase().as_str() {
            "none" => meeting.repeat = RepeatType::None,
            "weekly" => meeting.repeat = RepeatType::Weekly,
            "biweekly" => meeting.repeat = RepeatType::BiWeekly,
            "monthly" => meeting.repeat = RepeatType::Monthly,
            _ => {
                return "Repeat Type value unknown (options are None, Weekly, BiWeekly, Monthly)"
                    .to_string();
            }
        };
    } else {
        return "Repeat parsing error: ".to_string();
    }

    "Meeting created!\n".to_string() + &meeting.to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("meeting")
        .description("Create a meeting")
        .create_option(|option| {
            option
                .name("name")
                .description("Meeting Name")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("datetime")
                .description("Meeting Date and Time (UTC, %Y-%m-%d %H:%M:%S)")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("repeat")
                .description("Repeat Type (None, Weekly, BiWeekly, Monthly)")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
