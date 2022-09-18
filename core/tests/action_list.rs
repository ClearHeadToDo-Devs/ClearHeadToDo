use tabled::Table;
use uuid::Uuid;
use im::Vector;

use indoc::indoc;

use clear_head_todo_core::action_implementation::ActionImplementation;
use clear_head_todo_core::action::*;

fn create_single_action_list() -> Vector<Action> {
    let mut action_list = Vector::new();

    action_list.push_back(Action::default());

    return action_list;
}

fn invalid_index_error_string(index: usize) -> String {
    return format!("No Action at Index {}", index);
}

#[test]
fn append_default() {
    let empty_action_list = Vector::new();

    let single_action_list = empty_action_list.append_default_action();

    assert_eq!(single_action_list.len(), 1);
}

#[test]
fn action_successful_search_by_id_test() {
    let action_list = create_single_action_list();

    let test_search_action = action_list
        .select_action_by_id(action_list[0].get_id()).unwrap();

    assert!(test_search_action == action_list[0]);
}

#[test]
fn failed_search_by_id() {
    let empty_list = Vector::new();

    let test_search_action = empty_list.select_action_by_id(Uuid::nil());
    let error_message = test_search_action.unwrap_err().to_string();

    assert_eq!(error_message,"No Action with Id 00000000-0000-0000-0000-000000000000");
}

#[test]
fn successful_select_by_index() {
    let single_action_list = create_single_action_list();

    let test_search_action = single_action_list.select_action_by_index(0).unwrap();

    assert!(test_search_action == single_action_list[0]);
}

#[test]
fn failed_select_by_index() {
    let empty_list = Vector::new();

    let failed_action_selection = empty_list.select_action_by_index(0).unwrap_err();

    assert_eq!(failed_action_selection.to_string() , invalid_index_error_string(0));
}

#[test]
fn action_print_empty_test() {
    let empty_action_list: Vector<Action> = Vector::new();

    let empty_list_str = format!("{:?}", empty_action_list);

    assert_eq!(empty_list_str.to_string(), "[]");
}

#[test]
fn action_print_successful_test() {
    let action_list = create_single_action_list();


    assert_eq!(format!("{:#?}", action_list),
            format!(
"[
    Action {{
        name: \"Default Action\",
        priority: Optional,
        completed: false,
        id: {},
    }},
]",action_list[0].get_id().simple()));
    }

#[test]
fn action_print_table_successful() {
    let action_list = create_single_action_list();

    let table = Table::new(action_list.clone());

    assert_eq!(table.to_string(),indoc!("
        +----------------+----------+-----------+
        | Name           | Priority | Completed |
        +----------------+----------+-----------+
        | Default Action | Optional | false     |
        +----------------+----------+-----------+"));
    }

#[test]
fn failing_action_removal_test() {
    let empty_action_list = Vector::new();

    let index_error = empty_action_list.remove_action(0).unwrap_err();

    assert_eq!(index_error.to_string(), invalid_index_error_string(0));
}

#[test]
fn successful_action_removal_test() {
    let action_list = create_single_action_list();

    let empty_list = action_list.remove_action(0).unwrap();

    assert!(empty_list.is_empty());
}

#[test]
fn action_completion_test() {
    let action_list = create_single_action_list();

    let good_result = action_list.toggle_action_completion_status(0).unwrap();

    assert_eq!(good_result[0].get_completion_status() , true);
}

#[test]
fn failing_action_completion_test() {
    let empty_list = Vector::new();

    let index_error = empty_list.toggle_action_completion_status(0).unwrap_err();

    assert_eq!(index_error.to_string(), invalid_index_error_string(0));
}

#[test]
fn action_reopen() {
    let action_list = create_single_action_list();

    let updated_action_list = &action_list
        .toggle_action_completion_status(0).unwrap()
        .toggle_action_completion_status(0).unwrap();

    assert_eq!(updated_action_list[0].get_completion_status(), false);
}

#[test]
fn action_rename() {
    let single_action_list = create_single_action_list();

    let good_result = &single_action_list
        .rename_action(0, "Changed Task".to_string())
        .unwrap();

    assert!(good_result[0].get_name() == "Changed Task".to_string());
}

#[test]
fn failing_action_rename() {
    let empty_list = Vector::new();

    let index_error = empty_list.rename_action(0, "Changed Task".to_string()).unwrap_err();

    assert_eq!(index_error.to_string(), invalid_index_error_string(0));
}

#[test]
fn action_reprioritize() {
    let single_action_list = create_single_action_list();

    let changed_action_list = &single_action_list
        .change_action_priority(0, "low".to_string())
        .unwrap();

    assert_eq!(changed_action_list[0].get_priority(), "Low");
}

#[test]
fn failed_action_reprioritize() {
    let empty_list = Vector::new();

    let index_error = empty_list.change_action_priority(0, "low".to_string()).unwrap_err();

    assert_eq!(index_error.to_string(), invalid_index_error_string(0));
}

#[test]
fn get_name(){
    let single_action_list = create_single_action_list();

    let name = single_action_list.get_action_name(0).unwrap();

    assert_eq!(name.to_string(), "Default Action");
}

#[test]
fn get_priority(){
    let single_action_list = create_single_action_list();

    let priority = single_action_list.get_action_priority(0).unwrap();

    assert_eq!(priority.to_string(), "Optional");
}

#[test]
fn get_completion_status(){
    let single_action_list = create_single_action_list();

    let completion_status = single_action_list.get_action_completion_status(0).unwrap();

    assert_eq!(completion_status, false);
}
