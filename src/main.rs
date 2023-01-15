mod action;
use action::*;

mod action_builder;
use action_builder::*;

mod action_interface;
use action_interface::*;

pub mod priority;
use action_interface::*;

fn main() {
    print!("Hello, world!");
}
