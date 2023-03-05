use crate::action::priority::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Add(AddTypes),
    List {
        #[arg(short, long)]
        full: bool,
    },
    #[command(subcommand)]
    Update(ActionUpdate),
}

#[derive(Subcommand)]
pub enum AddTypes {
    Action {
        name: Option<String>,
        priority: Option<Priority>,
        completed: Option<bool>,
    },
    Relationship {
        source: usize,
        target: usize,
        variant: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ActionUpdate {
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
        #[arg(short, long)]
        new_completion_status: bool,
    },
}
