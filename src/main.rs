mod action;
use action::*;
mod action_builder;
use action_builder::*;
mod action_interface;
use action_interface::*;
pub mod priority;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: Option<String> },
    List,
}

fn main() {
    let cli = Cli::parse();

    let mut action_list: Vec<Action> = vec![];

    match &cli.command {
        Commands::Add { name } => {
            let new_name = name.clone().unwrap();
            let new_action = ActionBuilder::default().set_name(&new_name).build();

            println!("Created {:?}", &new_action);

            action_list.push(new_action);
        }
        Commands::List => {
            println!("List")
        }
    }
}
