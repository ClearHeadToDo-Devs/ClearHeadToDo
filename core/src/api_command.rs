use action::ActionListManipulation;
use relationships::RelationshipListManagement;

use crate::ClearHeadApp;
use std::error::Error;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Command {
    List,
    ExtendedList,
    Create(Option<String>),
    CreateRelationship {
        variant: String,
        participant_1: usize,
        participant_2: usize,
    },
    ToggleCompletion(usize),
    Remove(usize),
    Rename { index: usize, new_name: String },
    Reprioritize { index: usize, new_priority: String },
}

impl Command {
    pub fn run_subcommand(
        &self,
        app: &ClearHeadApp,
    ) -> Result<ClearHeadApp, Box<dyn Error>> {
        match self {
            Command::List => {
                app.get_list();
                return Ok(app.clone());
            }
            Command::ExtendedList => {
                app.get_extended_list()?;
                return Ok(app.clone());
            }
            Command::Create(name) => {
                let updated_list = app.append_default();
                if let Some(name) = name {
                    return updated_list
                        .rename(updated_list.action_list.len() - 1, name.to_string());
                }
                Ok(updated_list)
            }
            Command::CreateRelationship {
                variant,
                participant_1,
                participant_2,
            } => {
                let updated_list = app.create_action_relationship(variant, 
                        *participant_1, *participant_2)?;
                Ok(updated_list)
            }
            Command::ToggleCompletion(index) => {
                let updated_list = app.toggle_completion_status(*index)?;
                Ok(updated_list)
            }
            Command::Remove(index) => {
                let updated_list = app.remove_action(*index)?;
                Ok(updated_list)
            }
            Command::Rename { index, new_name } => {
                let updated_list = app.rename(*index, new_name.to_string())?;
                Ok(updated_list)
            }
            Command::Reprioritize {
                index,
                new_priority,
            } => {
                let updated_list =
                    app.change_priority(*index, new_priority.to_string())?;
                Ok(updated_list)
            }
        }
    }

    pub fn create_end_user_message(
        &self,
        previous_app: &ClearHeadApp,
        updated_app: &ClearHeadApp,
    ) -> String {
        match self {
            Command::Create(_name) => {
                format!(
                    "Created Action {}",
                    updated_app.action_list[updated_app.action_list.len() - 1].get_name()
                )
            }
            Command::CreateRelationship {
                variant,
                participant_1,
                participant_2,
            } => {
                format!(
                    "Created {} Relationship from Action {} to Action {}",
                    variant, participant_1, participant_2
                )
            }
            Command::ToggleCompletion(index) => {
                format!(
                    "{} had its' completion status toggled to {}",
                    updated_app.action_list[*index].get_name(), updated_app.action_list[*index].completed
                )
            }
            Command::Remove(index) => {
                format!(
                    "{} was removed from your Action List",
                    previous_app.action_list[*index].get_name()
                )
            }
            Command::Rename { index, new_name } => {
                format!(
                    "{} was changed from {}",
                    new_name, previous_app.action_list[*index].get_name()
                )
            }
            Command::Reprioritize {
                index,
                new_priority,
            } => {
                format!(
                    "{} was changed from a priority of: {}\n to a priority of: {}",
                    updated_app.action_list[*index].get_name(),
                    previous_app.action_list[*index].priority,
                    new_priority
                )
            }
            Command::List => unreachable!(),
            Command::ExtendedList => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Priority;

    use crate::functionality::tests::create_app_with_single_action;

    #[test]
    fn list_failure_empty_list() {
        let empty_list: ClearHeadApp = Default::default();

        let error = Command::List.run_subcommand(&empty_list);
        assert_eq!(format!("{:?}",error.unwrap()), "ClearHeadApp { action_list: [], relationship_list: [] }");
    }

    #[test]
    fn create_successful_run() {
        let empty_list: ClearHeadApp = Default::default();

        let result = Command::Create(None)
            .run_subcommand(&empty_list)
            .unwrap();

        assert_eq!(result.action_list[0].get_name(), "Default Action".to_string());
        assert_eq!(result.action_list[0].priority, Priority::Optional);
        assert_eq!(result.action_list[0].completed, false);
    }

    #[test]
    fn generate_create_success_message() {
        let empty_list: ClearHeadApp = Default::default();
        let single_list = empty_list.append_default();

        let message =
            Command::Create(None).create_end_user_message(&empty_list, &single_list);
        assert_eq!(message, "Created Action Default Action");
    }

    #[test]
    fn complete_successful_run() {
        let empty_list: ClearHeadApp = Default::default();
        let single_action_lst = empty_list.append_default();

        let result = Command::ToggleCompletion(0).run_subcommand(&single_action_lst);

        assert_eq!(
            result.unwrap().action_list[0].completed,
            true
        );
    }

    #[test]
    fn reopen_successful_run() {
        let empty_app: ClearHeadApp = Default::default();
        let single_action_app = empty_app.append_default();
        let single_completed_action_app = single_action_app.toggle_completion_status(0).unwrap();

        let updated_list =
            Command::ToggleCompletion(0).run_subcommand(&single_completed_action_app).unwrap();

        assert_eq!(
            updated_list.action_list[0].completed,
            false
        );
    }

    #[test]
    fn generate_complete_message() {
        let empty_app = ClearHeadApp::default();
        let single_action_app = empty_app.append_default();
        let updated_action_app = single_action_app.toggle_completion_status(0).unwrap();

        let message = Command::ToggleCompletion(0)
            .create_end_user_message(&single_action_app, &updated_action_app);

        assert_eq!(
            message,
            "Default Action had its\' completion status toggled to true"
        );
    }

    #[test]
    fn complete_failing_invalid_id() {
        let empty_list: ClearHeadApp = Default::default();

        let error = Command::ToggleCompletion(1).run_subcommand(&empty_list);
        assert_eq!(error.unwrap_err().to_string(), "No Action at Index 1");
    }

    #[test]
    fn cli_remove_successful_run_test() {
        let empty_list: ClearHeadApp = Default::default();
        let single_list = empty_list.append_default();

        let result = Command::Remove(0).run_subcommand(&single_list);
        assert_eq!(result.unwrap(), ClearHeadApp::default());
    }

    #[test]
    fn generate_remove_message() {
        let empty_app = ClearHeadApp::default();
        let single_action_app = empty_app.append_default();
        let updated_app_list = single_action_app.remove_action(0).unwrap();

        let message =
            Command::Remove(0).create_end_user_message(&single_action_app, &updated_app_list);

        assert_eq!(message, "Default Action was removed from your Action List");
    }

    #[test]
    fn failing_cli_remove_invalid_index_test() {
        let empty_list: ClearHeadApp = Default::default();

        let index_error = Command::Remove(0).run_subcommand(&empty_list).unwrap_err();

        assert_eq!(index_error.to_string(), "No Action at Index 0");
    }

    #[test]
    fn cli_rename_successful_run_test() {
        let empty_list: ClearHeadApp = Default::default();
        let single_list = empty_list.append_default();

        let result = Command::Rename {
            index: 0,
            new_name: "Test Rename".to_string(),
        }
        .run_subcommand(&single_list).unwrap();

        assert_eq!(
            result.action_list[0].get_name(),
            "Test Rename".to_string()
        );
    }

    #[test]
    fn generate_rename_message() {
        let empty_app = ClearHeadApp::default();
        let single_action_app = empty_app.append_default();
        let updated_list = single_action_app
            .rename(0, "New Name".to_string())
            .unwrap();

        let message = Command::Rename {
            index: 0,
            new_name: "New Name".to_string(),
        }
        .create_end_user_message(&single_action_app, &updated_list);

        assert_eq!(message, "New Name was changed from Default Action");
    }

    #[test]
    fn cli_rename_failing_invalid_id_test() {
        let empty_list: ClearHeadApp = Default::default();

        let error = Command::Rename {
            index: 0,
            new_name: "Test Rename".to_string(),
        }
        .run_subcommand(&empty_list);
        assert_eq!(error.unwrap_err().to_string(), "No Action at Index 0");
    }

    #[test]
    fn cli_change_priority_successful_run_test() {
        let empty_list: ClearHeadApp = Default::default();
        let single_list = empty_list.append_default();

        let result = Command::Reprioritize {
            index: 0,
            new_priority: "High".to_string(),
        }
        .run_subcommand(&single_list).unwrap();
        assert_eq!(result.action_list[0].priority, Priority::High);
    }

    #[test]
    fn generate_reprioritize_message() {
        let empty_app = ClearHeadApp::default();
        let single_action_app = empty_app.append_default();
        let updated_list = single_action_app
            .change_priority(0, "low".to_string())
            .unwrap();

        let message = Command::Reprioritize {
            index: 0,
            new_priority: "low".to_string(),
        }
        .create_end_user_message(&single_action_app, &updated_list);

        assert_eq!(
            message,
            "Default Action was changed from a priority of: Optional\n to a priority of: low"
        );
    }

    #[test]
    fn cli_reprioritize_failing_invalid_id_test() {
        let empty_list: ClearHeadApp = Default::default();

        let error = Command::Reprioritize {
            index: 1,
            new_priority: "High".to_string(),
        }
        .run_subcommand(&empty_list);
        assert_eq!(error.unwrap_err().to_string(), "No Action at Index 1");
    }

    #[test]
    fn cli_create_related_relationship_successful_run() {
        let empty_list: ClearHeadApp = Default::default();
        let updated_list = empty_list.append_default().append_default();

        let result = Command::CreateRelationship {
            variant: "related".to_string(),
            participant_1: 0,
            participant_2: 1,
        }
        .run_subcommand(&updated_list).unwrap();
        assert_eq!(result.relationship_list.len(), 1);
        assert_eq!(result.relationship_list[0].get_variant().to_string(),
            "Related: Undirected" );
        assert_eq!(result.relationship_list[0].get_participant_1(), result.action_list[0].id);
        assert_eq!(result.relationship_list[0].get_participant_2(), result.action_list[1].id);
    }

    #[test]
    fn cli_create_sequential_relationship_successful_run() {
        let empty_list: ClearHeadApp = Default::default();
        let updated_list = empty_list.append_default().append_default();

        let result = Command::CreateRelationship {
            variant: "sequential".to_string(),
            participant_1: 0,
            participant_2: 1,
        }
        .run_subcommand(&updated_list).unwrap();
        assert_eq!(result.relationship_list[0].get_variant().to_string(), "Sequential: Directed");
    }

    #[test]
    fn cli_create_parental_relationship_successful_run() {
        let empty_list: ClearHeadApp = Default::default();
        let updated_list = empty_list.append_default().append_default();

        let result = Command::CreateRelationship {
            variant: "parental".to_string(),
            participant_1: 0,
            participant_2: 1,
        }
        .run_subcommand(&updated_list).unwrap();
        assert_eq!(result.relationship_list[0].get_variant().to_string(), "Parental: Directed");
    }

    #[test]
    fn cli_create_relationship_successful_message() {
        let empty_list: ClearHeadApp = ClearHeadApp::default().append_default().append_default();
        let single_relationship_app = empty_list.create_action_relationship("related", 0, 1).unwrap();
        let updated_list = single_relationship_app.append_default().append_default();

        let result = Command::CreateRelationship {
            variant: "related".to_string(),
            participant_1: 0,
            participant_2: 1,
        }.create_end_user_message(&single_relationship_app, &updated_list);

        assert_eq!(result, "Created related Relationship from Action 0 to Action 1");
    }
}
