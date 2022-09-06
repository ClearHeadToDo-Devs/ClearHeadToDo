extern crate clap;
use clap::{command, AppSettings, Arg, ArgMatches, SubCommand};

use crate::Error;
use crate::command_runner::Command;



pub fn create_app() -> clap::Command<'static> {
    command!()
        .author("Darrion Burgess <darrionburgess@gmail.com>")
        .version("0.1.0")
        .about("can be used to manage every part of your productive life!")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("list").alias("lt"))
        .subcommand(SubCommand::with_name("extended_list").alias("el"))
        .subcommand(
            SubCommand::with_name("create_action")
                .arg(Arg::with_name("new_name").multiple(true)),
        )
        .subcommand(SubCommand::with_name("create_relationship")
            .arg(Arg::with_name("variant").required(true))
            .arg(Arg::with_name("participant_1").required(true))
            .arg(Arg::with_name("participant_2").required(true)))
        .subcommand(
            SubCommand::with_name("complete")
                .arg(Arg::with_name("index").required(true)),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .arg(Arg::with_name("index").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rename")
                .arg(Arg::with_name("index").required(true))
                .arg(Arg::with_name("new_name").required(true).multiple(true)),
        )
        .subcommand(
            SubCommand::with_name("reprioritize")
                .alias("rp")
                .arg(Arg::with_name("index").required(true))
                .arg(Arg::with_name("new_priority").required(true)),
        )
}

pub trait ArgumentParsing {
    fn parse_command(&self) -> Result<Command, Box<dyn Error>>;
    fn parse_index_for_subcommand(&self, subcommand_name: String) -> Result<usize, Box<dyn Error>>;
    fn parse_desired_name(&self, subcommand_name: String) -> Option<String>;
    fn parse_desired_priority(&self, subcommand_name: String) -> String;
}

impl ArgumentParsing for ArgMatches {
    fn parse_command(&self) -> Result<Command, Box<dyn Error>> {
        match self.subcommand_name() {
            Some("list") => Ok(Command::List),
            Some("extended_list") => Ok(Command::ExtendedList),
            Some("create_action") => Ok(Command::Create(
                self.parse_desired_name("create_action".to_string()),
            )),
            Some("create_relationship") => Ok(Command::CreateRelationship{
                variant: self.subcommand_matches("create_relationship".to_string())
                .unwrap()
                .value_of("variant")
                .unwrap()
                .to_string(),
                participant_1: self.subcommand_matches("create_relationship".to_string())
                .unwrap()
                .value_of("participant_1")
                .unwrap()
                .parse::<usize>()?,
                participant_2: self.subcommand_matches("create_relationship".to_string())
                .unwrap()
                .value_of("participant_2")
                .unwrap()
                .parse::<usize>()?
            }),
            Some("complete") => Ok(Command::ToggleCompletion(
                self.parse_index_for_subcommand("complete".to_string())?,
            )),
            Some("remove") => Ok(Command::Remove(
                self.parse_index_for_subcommand("remove".to_string())?,
            )),
            Some("rename") => Ok(Command::Rename {
                index: self.parse_index_for_subcommand("rename".to_string())?,
                new_name: self.parse_desired_name("rename".to_string()).unwrap(),
            }),
            Some("reprioritize") => Ok(Command::Reprioritize {
                index: self.parse_index_for_subcommand("reprioritize".to_string())?,
                new_priority: self.parse_desired_priority("reprioritize".to_string()),
            }),
            _ => unreachable!(),
        }
    }

    fn parse_index_for_subcommand(&self, subcommand_name: String) -> Result<usize, Box<dyn Error>> {
        Ok(self
            .subcommand_matches(subcommand_name)
            .ok_or("this is not one of the subcommands of the interface")?
            .value_of("index")
            .ok_or("incompatible value for subcommand")?
            .parse::<usize>()?)
    }

    fn parse_desired_name(&self, subcommand_name: String) -> Option<String> {
        match self.subcommand_matches(subcommand_name) {
            Some(arg_matches) => Some(arg_matches.get_one::<String>("new_name")?.to_string()),
            None => None,
        }
    }

    fn parse_desired_priority(&self, subcommand_name: String) -> String {
        self.subcommand_matches(subcommand_name)
            .unwrap()
            .value_of("new_priority")
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::ErrorKind;

    #[test]
    fn cli_creation_name_test() {
        let app = create_app();

        assert_eq!(&app.get_name(), &"clear_head_todo_cli");
    }

    #[test]
    fn cli_creation_author() {
        let app = create_app();

        assert_eq!(
            &app.get_author().unwrap(),
            &"Darrion Burgess <darrionburgess@gmail.com>"
        );
    }

    #[test]
    fn cli_creation_version() {
        let app = create_app();

        assert_eq!(app.get_version().unwrap(), "0.1.0");
    }

    #[test]
    fn cli_creation_about() {
        let app = create_app();

        assert_eq!(
            app.get_about().unwrap(),
            "can be used to manage every part of your productive life!"
        );
    }

    #[test]
    fn cli_creation_subcommand_or_help() {
        let app = create_app();

        let matches = app.get_matches_from_safe(&[""]);
        let error = matches.unwrap_err();

        assert_eq!(
            error.kind,
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
    }

    #[test]
    fn cli_list_successful_match() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "list"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::List);
    }

    #[test]
    fn cli_list_alias() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "lt"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::List);
    }

    #[test]
    fn cli_extended_list_successful_match() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "extended_list"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::ExtendedList);
    }

    #[test]
    fn cli_extended_list_alias() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "el"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::ExtendedList);
    }

    #[test]
    fn cli_create_successful_parse() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "create_action", "Test Task"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::Create(Some("Test Task".to_string())));
    }

    #[test]
    fn cli_create_alias() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "create_action"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::Create(None));
    }

    #[test]
    fn cli_create_relationship_successful_parse() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "create_relationship", "related", "0", "1"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::CreateRelationship{
            variant: "related".to_owned(), 
            participant_1: 0,
            participant_2: 1});
    }

    #[test]
    fn cli_complete_successful_parse() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "complete", "0"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::ToggleCompletion(0));
    }

    #[test]
    fn cli_complete_alias() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "complete", "1"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::ToggleCompletion(1));
    }

    #[test]
    fn cli_remove_successful_parse() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "remove", "1"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::Remove(1));
    }

    #[test]
    fn cli_remove_alias() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "remove", "0"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::Remove(0));
    }

    #[test]
    fn successful_cli_rename_parse() {
        let app = create_app();
        let test_matches =
            app.get_matches_from(vec!["ClearHeadToDo", "rename", "0", "Test Rename"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(
            result,
            Command::Rename {
                index: 0,
                new_name: "Test Rename".to_string()
            }
        );
    }

    #[test]
    fn cli_rename_alias() {
        let app = create_app();
        let test_matches =
            app.get_matches_from(vec!["ClearHeadToDo", "rename", "0", "Test Rename"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(
            result,
            Command::Rename {
                index: 0,
                new_name: "Test Rename".to_string()
            }
        );
    }

    #[test]
    fn cli_change_priority_successful_parse() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "reprioritize", "1", "High"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(
            result,
            Command::Reprioritize {
                index: 1,
                new_priority: "High".to_string()
            }
        );
    }

    #[test]
    fn cli_reprioritize_alias() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "rp", "1", "High"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(
            result,
            Command::Reprioritize {
                index: 1,
                new_priority: "High".to_string()
            }
        );
    }
}
