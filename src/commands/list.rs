use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::types::meeting::{Meeting, RepeatType};
use std::sync::Mutex;

pub fn run(_options: &[CommandDataOption], meetings: &Mutex<Vec<Meeting>>) -> String {
    let mut repeated = String::from("**Repeating Meetings:**\n\n");
    let mut singular = String::from("**Other Meetings:**\n\n");

    for i in meetings.lock().unwrap().iter() {
        if i.repeat != RepeatType::None {
            repeated += &i.to_string();
            repeated.push('\n');
            repeated.push('\n');
        } else {
            singular += &i.to_string();
            singular.push('\n');
            singular.push('\n');
        }
    }
    /*
        for i in &mut meetings.lock().unwrap().deref_mut().iter_mut() {
            i.propogate_next();
            out += &i.to_string();
            out.push('\n');
            out.push('\n');
        }
    */
    repeated + &singular
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("list").description("List meetings")
    /*        .create_option(|option| {
        option
            .name("datetime")
            .description("Meeting Date and Time (UTC)")
            .kind(CommandOptionType::String)
            .required(true)
    })*/
}
