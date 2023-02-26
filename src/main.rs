mod action;
use action::*;
mod action_builder;
use action_builder::*;
mod action_interface;
use action_interface::*;
pub mod priority;
mod relationship;
use indradb::MemoryDatastore;
use relationship::*;

pub mod graph_storage;
use graph_storage::*;
use uuid::Uuid;

use clap::{Parser, Subcommand};

use crate::priority::Priority;

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
        priority: Option<Priority>,
        completed: Option<bool>,
    },
    List,
}

#[derive(Subcommand)]
enum AddCommands {
    Action {
        name: Option<String>,
        priority: Option<Priority>,
        completed: Option<bool>,
    },
    Relationship {
        source: Uuid,
        target: Uuid,
        variant: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut action_list: Vec<Action> = vec![];

    let mut datastore: MemoryDatastore = MemoryDatastore::default();

    match &cli.command {
        Commands::Add {
            name,
            priority,
            completed,
        } => {
            let new_name = name.clone().unwrap();
            let new_priority = priority.clone().unwrap_or(Priority::Optional);
            let completion_status = completed.unwrap_or(false);

            let new_action = ActionBuilder::default()
                .set_name(&new_name)
                .set_priority(new_priority)
                .set_completion_status(completion_status)
                .build();

            let add_result = add_action_to_datastore(new_action.clone(), datastore);

            println!("Created {:?}", &new_action);

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
