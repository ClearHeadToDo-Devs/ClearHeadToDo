mod action;
use action::*;

mod action_builder;
use action_builder::*;

mod action_interface;
use action_interface::*;

pub mod priority;
use action_interface::*;

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
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { name } => {
            let new_name = name.clone().unwrap();
            let new_action = ActionBuilder::default().set_name(&new_name).build();
            println!("Created {:?}", new_action)
        }
    }
}
