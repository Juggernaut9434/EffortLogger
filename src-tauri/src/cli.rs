use clap::{arg, Command}; // Import clap for the CLI build

use crate::story::{add_story_to_list, read_story_list_from_file};

pub fn run_cli() {
    // Define the CLI logic here using clap
    let matches = Command::new("Tauri CLI Example")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("An example Tauri CLI app")
        .subcommand_required(true)
        .subcommand(
            Command::new("start")
                .about("Start the clock on a story-id")
                .arg(arg!([ID])),
        )
        .subcommand(
            Command::new("stop")
                .about("Stop the clock on a story-id")
                .arg(arg!([ID])),
        )
        .subcommand(
            Command::new("story")
                .about("Manage stories")
                .subcommand(Command::new("list").about("List the stories present"))
                .subcommand(Command::new("add").about("Add a story").arg(arg!([ID]))),
        )
        .get_matches();

    // Check if the "start" subcommand was used
    match matches.subcommand() {
        Some(("start", sub_matches)) => {
            if let Some(story_id) = sub_matches.get_one::<String>("ID") {
                // Handle the "start" subcommand with the provided story-id
                println!("Starting story with ID: {}", story_id);
            }
        }
        Some(("stop", sub_matches)) => {
            if let Some(story_id) = sub_matches.get_one::<String>("ID") {
                // Handle the "start" subcommand with the provided story-id
                println!("Stopping story with ID: {}", story_id);
            }
        }
        Some(("story", sub_matches)) => match sub_matches.subcommand() {
            Some(("list", subsub_matches)) => {
                let stories = match read_story_list_from_file() {
                    Ok(stories) => stories,
                    Err(e) => vec![],
                };
                println!("Printing all Stories");
                // Print the stories
                if stories.is_empty() {
                    println!("No stories found.");
                } else {
                    println!("Printing all Stories:");
                    for story in stories {
                        println!("{}", story);
                    }
                }
            }
            Some(("add", subsub_matches)) => {
                if let Some(story_id) = subsub_matches.get_one::<String>("ID") {
                    println!("Adding a new story");
                    match add_story_to_list(story_id.to_string()) {
                        Ok(_) => println!("Successful story added"),
                        Err(e) => println!("Failed story add {}", e.to_string()),
                    }
                }
            }
            _ => unreachable!(
                "Exhausted list of subcommands and subcommand_required prevents `None`"
            ),
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
