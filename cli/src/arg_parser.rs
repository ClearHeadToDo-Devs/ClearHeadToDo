extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::Error;
use clear_head_todo_core::Command;

pub fn create_app() -> App<'static, 'static> {
    App::new("Clear Head Todo")
        .author("Darrion Burgess <darrionburgess@gmail.com>")
        .version("0.1.0")
        .about("can be used to manage every part of your productive life!")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("list_tasks").alias("lt"))
        .subcommand(SubCommand::with_name("create_task").alias("create"))
        .subcommand(
            SubCommand::with_name("complete_task")
                .alias("complete")
                .arg(Arg::with_name("index").required(true)),
        )
        .subcommand(
            SubCommand::with_name("remove_task")
                .alias("remove")
                .arg(Arg::with_name("index").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rename_task")
                .alias("rename")
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

impl ArgumentParsing for ArgMatches<'_> {
    fn parse_command(&self) -> Result<Command, Box<dyn Error>> {
        match self.subcommand_name() {
            Some("list_tasks") => Ok(Command::ListTasks),
            Some("create_task") => Ok(Command::CreateTask(
                self.parse_desired_name("create_task".to_string()),
            )),
            Some("complete_task") => Ok(Command::ToggleTaskCompletion(
                self.parse_index_for_subcommand("complete_task".to_string())?,
            )),
            Some("remove_task") => Ok(Command::RemoveTask(
                self.parse_index_for_subcommand("remove_task".to_string())?,
            )),
            Some("rename_task") => Ok(Command::RenameTask {
                index: self.parse_index_for_subcommand("rename_task".to_string())?,
                new_name: self.parse_desired_name("rename_task".to_string()).unwrap(),
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
        Some(
            self.subcommand_matches(subcommand_name)
                .unwrap()
                .values_of("new_name")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" ")
                .to_string(),
        )
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

        assert_eq!(&app.p.meta.name, &"Clear Head Todo");
    }

    #[test]
    fn cli_creation_author_test() {
        let app = create_app();

        assert_eq!(
            &app.p.meta.author.unwrap(),
            &"Darrion Burgess <darrionburgess@gmail.com>"
        );
    }

    #[test]
    fn cli_creation_version_test() {
        let app = create_app();

        assert_eq!(app.p.meta.version.unwrap(), "0.1.0");
    }

    #[test]
    fn cli_creation_about_test() {
        let app = create_app();

        assert_eq!(
            app.p.meta.about.unwrap(),
            "can be used to manage every part of your productive life!"
        );
    }

    #[test]
    fn cli_creation_subcommand_or_help_test() {
        let app = create_app();

        let matches = app.get_matches_from_safe(&[""]);
        let error = matches.unwrap_err();

        assert_eq!(error.kind, ErrorKind::MissingArgumentOrSubcommand);
    }

    #[test]
    fn cli_list_task_successful_match_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "list_tasks"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::ListTasks);
    }

    #[test]
    fn cli_list_task_alias_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "lt"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::ListTasks);
    }

    #[test]
    fn cli_create_task_successful_parse_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "create_task"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::CreateTask(None));
    }

    #[test]
    fn cli_create_task_alias_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "create"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::CreateTask(None));
    }

    #[test]
    fn cli_complete_task_successful_parse_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "complete_task", "0"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::ToggleTaskCompletion(0));
    }

    #[test]
    fn cli_complete_task_alias_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "complete", "1"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::ToggleTaskCompletion(1));
    }

    #[test]
    fn cli_remove_task_successful_parse_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "remove_task", "1"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::RemoveTask(1));
    }

    #[test]
    fn cli_remove_task_alias_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "remove", "0"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(result, Command::RemoveTask(0));
    }

    #[test]
    fn successful_cli_rename_task_parse_test() {
        let app = create_app();
        let test_matches =
            app.get_matches_from(vec!["ClearHeadToDo", "rename_task", "0", "Test", "Rename"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(
            result,
            Command::RenameTask {
                index: 0,
                new_name: "Test Rename".to_string()
            }
        );
    }

    #[test]
    fn cli_rename_task_alias_test() {
        let app = create_app();
        let test_matches =
            app.get_matches_from(vec!["ClearHeadToDo", "rename", "0", "Test Rename"]);

        let result = test_matches.parse_command().unwrap();
        assert_eq!(
            result,
            Command::RenameTask {
                index: 0,
                new_name: "Test Rename".to_string()
            }
        );
    }

    #[test]
    fn cli_change_priority_successful_parse_test() {
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
    fn cli_reprioritize_task_alias_test() {
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
