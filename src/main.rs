mod action;

use action::*;
mod action_builder;
use action_builder::*;
mod action_interface;
use action_interface::*;
pub mod priority;

mod relationship;
use relationship::*;

mod graph_storage;
use file_management::*;
use graph_storage::*;

use clap::{Parser, Subcommand};
use indradb::{Datastore, MemoryDatastore};

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
    #[command(subcommand)]
    Add(AddTypes),
    List,
    #[command(subcommand)]
    Update(ActionUpdate),
}

#[derive(Subcommand)]
enum AddTypes {
    Action {
        name: Option<String>,
        priority: Option<Priority>,
        completed: Option<bool>,
    },
}

#[derive(Subcommand)]
enum ActionUpdate {
    Name {
        index: usize,
        new_name: String,
    },
    Priority {
        index: usize,
        new_priority: Priority,
    },
    Completed {
        index: usize,
        new_completion_status: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let datastore: MemoryDatastore = get_clearhead_datastore("clearhead.db");

    match &cli.command {
        Commands::Add(add) => match add {
            AddTypes::Action {
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

                let (updated_datastore, _) =
                    add_action_to_datastore(new_action.clone(), datastore)?;

                updated_datastore.sync().unwrap();

                println!("Created {:?}", &new_action);

                Ok(())
            }
        },
        Commands::Update(update) => match update {
            ActionUpdate::Name {
                index: id,
                new_name,
            } => {
                let action_list = get_all_actions_from_datastore(&datastore);
                let updated_datastore =
                    update_action_vertex_name(datastore, action_list[id - 1].get_id(), new_name)?;

                updated_datastore.sync().unwrap();

                println!("Updated {:?}", action_list[id - 1]);

                Ok(())
            }
            &ActionUpdate::Priority {
                index: id,
                new_priority,
            } => {
                let action_list = get_all_actions_from_datastore(&datastore);
                let updated_datastore = update_action_vertex_priority(
                    datastore,
                    action_list[id - 1].get_id(),
                    new_priority,
                )?;

                updated_datastore.sync().unwrap();

                println!("Updated {:?}", action_list[id - 1]);

                Ok(())
            }
            &ActionUpdate::Completed {
                index: id,
                new_completion_status,
            } => {
                let action_list = get_all_actions_from_datastore(&datastore);
                let updated_datastore = update_action_vertex_completion_status(
                    datastore,
                    action_list[id - 1].get_id(),
                    new_completion_status,
                )?;

                updated_datastore.sync().unwrap();

                println!("Updated {:?}", action_list[id - 1]);

                Ok(())
            }
        },

        Commands::List => {
            let action_list: Vec<Action> = get_all_actions_from_datastore(&datastore);

            for action in action_list.clone() {
                println!(
                    "{}. {}, Priority: {}, Completed: {}",
                    action_list.iter().position(|a| a == &action).unwrap() + 1,
                    action.get_name(),
                    action.get_priority(),
                    action.get_completion_status()
                );
            }

            Ok(())
        }
    }
}
