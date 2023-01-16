mod action;
use action::*;
mod action_builder;
use action_builder::*;
mod action_interface;
use action_interface::*;
pub mod priority;
mod file_management;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: Option<String>,
        priority: Option<String>,
        completed: Option<bool>,
    },
    List,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut action_list: Vec<Action> = vec![];

    match &cli.command {
        Commands::Add { name, priority, completed} => {
            let new_name = name.clone().unwrap();
            let new_priority = priority.clone().unwrap_or("Optional".to_string());
            let completion_status = completed.unwrap_or(false);

            let new_action = ActionBuilder::default()
                .set_name(&new_name)
                .set_priority(&new_priority)
                .unwrap()
                .set_completion_status(completion_status)
                .build();

            println!("Created {:?}", &new_action);

            action_list.push(new_action);

            Ok(())
        }
        Commands::List => {
            for action in action_list {
                println!("{:?}", &action);
            }

            Ok(())
        }
    }
}
