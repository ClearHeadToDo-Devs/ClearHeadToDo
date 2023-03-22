use clap::Parser;
mod action;
use action::builder::*;
use action::interface::*;
use action::priority::*;
use action::Action;

use indradb::SpecificEdgeQuery;
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

use petgraph::dot::Dot;
use petgraph::stable_graph::StableGraph;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let datastore: MemoryDatastore = get_clearhead_datastore("clearhead.db");
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(add) => match add {
            AddCommands::Action {
                name,
                priority,
                completed,
            } => {
                let name = name.clone().unwrap();
                let new_priority = priority.clone().unwrap_or(Priority::Optional);
                let completion_status = completed.unwrap_or(false);

                let new_action = ActionBuilder::default()
                    .set_name(&name)
                    .set_priority(new_priority)
                    .set_completion_status(completion_status)
                    .build();

                let (updated_datastore, _) =
                    add_action_to_datastore(new_action.clone(), datastore)?;

                updated_datastore.sync().unwrap();

                println!("Created {:?}", &new_action);

                Ok(())
            }
            AddCommands::Relationship {
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
            ActionUpdate::Name { index, new_name } => {
                let action_list = get_all_actions_from_datastore(&datastore);
                let updated_datastore = update_action_vertex_name(
                    datastore,
                    action_list[index - 1].get_id(),
                    new_name,
                )?;

                updated_datastore.sync().unwrap();

                println!("Updated {:?}", action_list[index - 1]);

                Ok(())
            }
            &ActionUpdate::Priority {
                index,
                new_priority,
            } => {
                let action_list = get_all_actions_from_datastore(&datastore);
                let updated_datastore = update_action_vertex_priority(
                    datastore,
                    action_list[index - 1].get_id(),
                    new_priority,
                )?;

                updated_datastore.sync().unwrap();

                println!("Updated {:?}", action_list[index - 1]);

                Ok(())
            }
            &ActionUpdate::Completed {
                index,
                new_completion_status,
            } => {
                let action_list = get_all_actions_from_datastore(&datastore);
                let updated_datastore = update_action_vertex_completion_status(
                    datastore,
                    action_list[index - 1].get_id(),
                    new_completion_status,
                )?;

                updated_datastore.sync().unwrap();

                println!("Updated {:?}", action_list[index - 1]);

                Ok(())
            }
        },

        Commands::List { full } => {
            let action_list: Vec<Action> = get_all_actions_from_datastore(&datastore);

            match full {
                true => {
                    let mut graph = StableGraph::<Action, RelationshipVariant>::new();

                    for action in action_list.clone() {
                        let action_node = graph.add_node(action.clone());
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

                                let related_action_node = graph.add_node(related_action.clone());

                                graph.add_edge(
                                    action_node,
                                    related_action_node,
                                    RelationshipVariant::from_str(&edge.key.t.to_string()).unwrap(),
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
                    println!("{:?}", Dot::with_config(&graph, &[]));
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
        Commands::Delete(delete) => match delete {
            DeleteCommands::Action { index } => {
                let action_list = get_all_actions_from_datastore(&datastore);
                let updated_datastore =
                    delete_action_from_datastore(datastore, action_list[index - 1].get_id())?;

                updated_datastore.sync().unwrap();

                println!("Deleted {:?}", action_list[index - 1]);

                Ok(())
            }
            DeleteCommands::Relationship { index_1, index_2 } => {
                let action_list = get_all_actions_from_datastore(&datastore);

                let id_1 = action_list[index_1 - 1].get_id();
                let id_2 = action_list[index_2 - 1].get_id();

                let id_1_relationship_list = datastore
                    .get_edges(SpecificVertexQuery::single(id_1).outbound().into())
                    .unwrap();

                let target_relationship = id_1_relationship_list
                    .iter()
                    .find(|relationship| relationship.key.inbound_id == id_2)
                    .unwrap();

                datastore
                    .delete_edges(SpecificEdgeQuery::single(target_relationship.key.clone()).into())
                    .unwrap();

                Ok(())
            }
        },
    }
}
