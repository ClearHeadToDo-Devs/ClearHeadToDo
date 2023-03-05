use clap::Parser;
mod action;
use action::builder::*;
use action::interface::*;
use action::priority::*;
use action::Action;

use uuid::Uuid;

mod relationship;
use relationship::*;

mod graph_storage;
use file_management::*;
use graph_storage::*;

use indradb::{Datastore, EdgeKey, MemoryDatastore, SpecificVertexQuery, VertexQueryExt};

pub mod arg_parse;
use arg_parse::*;
use std::str::FromStr;

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
            AddTypes::Relationship {
                variant,
                source,
                target,
            } => {
                let action_list = get_all_actions_from_datastore(&datastore);

                let variant_string = variant.clone().unwrap_or("".to_string());

                let rel_variant: RelationshipVariant =
                    RelationshipVariant::from_str(&variant_string)
                        .unwrap_or(RelationshipVariant::default());

                let new_relationship = Relationship::new(
                    Uuid::nil(),
                    Some(rel_variant),
                    action_list[*target].get_id(),
                    action_list[*source].get_id(),
                );

                let edge_key: EdgeKey = new_relationship.clone().into();

                match datastore.create_edge(&edge_key) {
                    Ok(_) => println!("Created {:?}", &new_relationship),
                    Err(e) => println!("Error: {:?}", e),
                }

                datastore.sync().unwrap();
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

        Commands::List { full } => {
            let action_list: Vec<Action> = get_all_actions_from_datastore(&datastore);

            match full {
                true => {
                    for action in action_list.clone() {
                        println!(
                            "{}. {}, Priority: {}, Completed: {}",
                            action_list.iter().position(|a| a == &action).unwrap() + 1,
                            action.get_name(),
                            action.get_priority(),
                            action.get_completion_status()
                        );
                        if datastore
                            .get_edge_count(action.get_id(), None, indradb::EdgeDirection::Outbound)
                            .unwrap()
                            > 0
                        {
                            let edge_list = datastore
                                .get_edges(
                                    SpecificVertexQuery::single(action.get_id())
                                        .outbound()
                                        .into(),
                                )
                                .unwrap();

                            for edge in edge_list.clone() {
                                let related_action = Action::from(
                                    datastore
                                        .get_all_vertex_properties(
                                            SpecificVertexQuery::single(edge.key.inbound_id).into(),
                                        )
                                        .unwrap()[0]
                                        .clone(),
                                );
                                println!("Relationships:");
                                println!(
                                    "  {}. {}, {}, Priority: {}",
                                    edge_list.iter().position(|a| a == &edge).unwrap() + 1,
                                    edge.key.t.as_str(),
                                    related_action.get_name(),
                                    related_action.get_priority().to_string()
                                )
                            }
                        }
                    }
                }
                false => {
                    for action in action_list.clone() {
                        println!(
                            "{}. {}, Priority: {}, Completed: {}",
                            action_list.iter().position(|a| a == &action).unwrap() + 1,
                            action.get_name(),
                            action.get_priority(),
                            action.get_completion_status()
                        );
                    }
                }
            };

            Ok(())
        }
    }
}
