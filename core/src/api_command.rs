use crate::item::Action;
use crate::ActionListManipulation;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Command {
    List,
    Create(Option<String>),
    ToggleCompletion(usize),
    Remove(usize),
    Rename { index: usize, new_name: String },
    Reprioritize { index: usize, new_priority: String },
}

impl Command {
    pub fn run_subcommand(
        &self,
        task_list: &im::Vector<Action>,
    ) -> Result<im::Vector<Action>, Box<dyn Error>> {
        match self {
            Command::List => {
                task_list.print_list()?;
                return Ok(task_list.clone());
            }
            Command::Create(name) => {
                let updated_list = task_list.create_new();
                if let Some(name) = name {
                    return updated_list
                        .rename(updated_list.len() - 1, name.to_string());
                }
                Ok(updated_list)
            }
            Command::ToggleCompletion(index) => {
                let updated_list = task_list.toggle_completion_status(*index)?;
                Ok(updated_list)
            }
            Command::Remove(index) => {
                let updated_list = task_list.remove(*index)?;
                Ok(updated_list)
            }
            Command::Rename { index, new_name } => {
                let updated_list = task_list.rename(*index, new_name.to_string())?;
                Ok(updated_list)
            }
            Command::Reprioritize {
                index,
                new_priority,
            } => {
                let updated_list =
                    task_list.change_priority(*index, new_priority.to_string())?;
                Ok(updated_list)
            }
        }
    }

    pub fn create_end_user_message(
        &self,
        previous_list: &im::Vector<Action>,
        updated_list: &im::Vector<Action>,
    ) -> String {
        match self {
            Command::Create(_name) => {
                format!(
                    "Created Task {}",
                    updated_list[updated_list.len() - 1].name
                )
            }
            Command::ToggleCompletion(index) => {
                format!(
                    "{} had its' completion status toggled to {}",
                    updated_list[*index].name, updated_list[*index].completed
                )
            }
            Command::Remove(index) => {
                format!(
                    "{} was removed from your Task List",
                    previous_list[*index].name
                )
            }
            Command::Rename { index, new_name } => {
                format!(
                    "{} was changed from {}",
                    new_name, previous_list[*index].name
                )
            }
            Command::Reprioritize {
                index,
                new_priority,
            } => {
                format!(
                    "{} was changed from a priority of: {}\n to a priority of: {}",
                    updated_list[*index].name,
                    previous_list[*index].priority,
                    new_priority
                )
            }
            Command::List => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::add_nil_action;
    use crate::Priority;
    use crate::Action;
    use im::vector;
    use uuid::Uuid;

    #[test]
    fn list_failure_empty_list() {
        let empty_list = vector!();

        let error = Command::List.run_subcommand(&empty_list);
        assert_eq!(error.unwrap_err().to_string(), "list is empty");
    }

    #[test]
    fn create_successful_run() {
        let empty_list = vector!();
        let result = Command::Create(None)
            .run_subcommand(&empty_list)
            .unwrap();

        assert_eq!(result[0].name, "Default Action".to_string());
        assert_eq!(result[0].priority, Priority::Optional);
        assert_eq!(result[0].completed, false);
    }

    #[test]
    fn generate_create_success_message() {
        let empty_list = vector!();
        let single_list = empty_list.create_new();

        let message =
            Command::Create(None).create_end_user_message(&empty_list, &single_list);
        assert_eq!(message, "Created Task Default Action");
    }

    #[test]
    fn complete_successful_run() {
        let single_list = add_nil_action(im::vector!());

        let result = Command::ToggleCompletion(0).run_subcommand(&single_list);

        assert_eq!(
            result.unwrap(),
            vector![Action {
                completed: true,
                id: Uuid::nil(),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn reopen_successful_run() {
        let single_completed_list = vector![Action {
            completed: true,
            id: Uuid::nil(),
            ..Default::default()
        }];

        let updated_list =
            Command::ToggleCompletion(0).run_subcommand(&single_completed_list);
        assert_eq!(
            updated_list.unwrap(),
            vector![Action {
                id: Uuid::nil(),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn generate_complete_message() {
        let single_list = vector!().create_new();
        let updated_list = single_list.toggle_completion_status(0).unwrap();

        let message = Command::ToggleCompletion(0)
            .create_end_user_message(&single_list, &updated_list);

        assert_eq!(
            message,
            "Default Action had its\' completion status toggled to true"
        );
    }

    #[test]
    fn complete_failing_invalid_id() {
        let empty_list = vector!();

        let error = Command::ToggleCompletion(1).run_subcommand(&empty_list);
        assert_eq!(error.unwrap_err().to_string(), "No Action at Index 1");
    }

    #[test]
    fn cli_remove_successful_run_test() {
        let empty_list = vector!();
        let single_list = empty_list.create_new();

        let result = Command::Remove(0).run_subcommand(&single_list);
        assert_eq!(result.unwrap(), vector!());
    }

    #[test]
    fn generate_remove_message() {
        let single_list = vector!().create_new();
        let updated_list = single_list.remove(0).unwrap();

        let message =
            Command::Remove(0).create_end_user_message(&single_list, &updated_list);

        assert_eq!(message, "Default Action was removed from your Task List");
    }

    #[test]
    fn failing_cli_remove_invalid_index_test() {
        let test_list = vector!();

        let error = Command::Remove(0).run_subcommand(&test_list);
        assert_eq!(error.unwrap_err().to_string(), "No Action at Index 0");
    }

    #[test]
    fn cli_rename_successful_run_test() {
        let empty_list = vector!();
        let single_list = add_nil_action(empty_list);

        let result = Command::Rename {
            index: 0,
            new_name: "Test Rename".to_string(),
        }
        .run_subcommand(&single_list);

        assert_eq!(
            result.unwrap(),
            vector![Action {
                name: "Test Rename".to_string(),
                id: Uuid::nil(),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn generate_rename_message() {
        let single_list = vector!().create_new();
        let updated_list = single_list
            .rename(0, "New Name".to_string())
            .unwrap();

        let message = Command::Rename {
            index: 0,
            new_name: "New Name".to_string(),
        }
        .create_end_user_message(&single_list, &updated_list);

        assert_eq!(message, "New Name was changed from Default Action");
    }

    #[test]
    fn cli_rename_failing_invalid_id_test() {
        let test_list = vector!();

        let error = Command::Rename {
            index: 0,
            new_name: "Test Rename".to_string(),
        }
        .run_subcommand(&test_list);
        assert_eq!(error.unwrap_err().to_string(), "No Action at Index 0");
    }

    #[test]
    fn cli_change_priority_successful_run_test() {
        let empty_list = vector!();
        let single_list = add_nil_action(empty_list);

        let result = Command::Reprioritize {
            index: 0,
            new_priority: "High".to_string(),
        }
        .run_subcommand(&single_list);
        assert_eq!(
            result.as_ref().unwrap(),
            &vector![Action {
                priority: Priority::High,
                id: Uuid::nil(),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn generate_reprioritize_message() {
        let single_list = vector!().create_new();
        let updated_list = single_list
            .change_priority(0, "low".to_string())
            .unwrap();

        let message = Command::Reprioritize {
            index: 0,
            new_priority: "low".to_string(),
        }
        .create_end_user_message(&single_list, &updated_list);

        assert_eq!(
            message,
            "Default Action was changed from a priority of: Optional\n to a priority of: low"
        );
    }

    #[test]
    fn cli_reprioritize_failing_invalid_id_test() {
        let empty_list = vector!();

        let error = Command::Reprioritize {
            index: 1,
            new_priority: "High".to_string(),
        }
        .run_subcommand(&empty_list);
        assert_eq!(error.unwrap_err().to_string(), "No Action at Index 1");
    }
}
