mod action;
use action::*;
mod action_builder;
use action_builder::*;
mod action_interface;
use action_interface::*;
pub mod priority;
mod relationship;
use indradb::{Datastore, MemoryDatastore, RangeVertexQuery, SpecificVertexQuery};
use relationship::*;

pub mod graph_storage;
use graph_storage::file_management::*;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut action_list: Vec<Action> = vec![];

    let mut datastore: MemoryDatastore = get_clearhead_datastore();

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

            let (updated_datastore, action_uuid) =
                add_action_to_datastore(new_action.clone(), datastore)?;

            updated_datastore.sync().unwrap();

            println!("Created {:?}", &new_action);

            Ok(())
        }
        Commands::List => {
            let property_list =
                datastore.get_all_vertex_properties(RangeVertexQuery::new().into())?;

            let mut action_list: Vec<Action> = vec![];

            for vertex in property_list {
                let mut builder = ActionBuilder::default();

                let new_action = builder
                    .set_completion_status(vertex.props[0].value.as_bool().unwrap())
                    .set_name(vertex.props[1].value.as_str().unwrap())
                    .set_priority(vertex.props[2].value.as_u64().unwrap().into())
                    .set_id(vertex.vertex.id)
                    .build();

                action_list.push(new_action);
            }

            for action in action_list {
                println!("{:?}", action);
            }

            Ok(())
        }
    }
}
