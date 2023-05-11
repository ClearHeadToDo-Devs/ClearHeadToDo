use clap::Parser;
mod action;
use action::builder::*;
use action::interface::*;
use action::priority::*;
use action::Action;

mod relationship;

mod graph_storage;
use file_management::*;
use graph_storage::*;

use indradb::MemoryDatastore;

pub mod arg_parse;
use arg_parse::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let datastore: MemoryDatastore = get_clearhead_datastore("clearhead.db");

    let cli = Cli::parse();

    run_arg_commands(cli, datastore)
}
