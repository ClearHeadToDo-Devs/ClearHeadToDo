use std::error::Error;
use std::str::FromStr;

use tabled::Table;
use uuid::Uuid;
use im::vector;

use action::*;

#[test]
fn append_default() {
    let empty_action_list: im::Vector<Action> = vector!();

    let single_action_list = empty_action_list.create_new();

    assert_eq!(single_action_list.len(), 1);
}

#[test]
fn action_successful_search_by_id_test() -> Result<(), Box<dyn Error>> {
    let empty_action_list: im::Vector<Action> = vector!();
    let single_nil_action_list = empty_action_list.create_new();
    let test_search_action = single_nil_action_list.select_by_id(single_nil_action_list[0].get_id())?;

    assert!(test_search_action == single_nil_action_list[0]);

    return Ok(());
}

#[test]
fn failed_search_by_id() {
    let empty_action_list: im::Vector<Action> = vector!();

    let test_search_action = empty_action_list
        .select_by_id(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
    let error_message = test_search_action.unwrap_err().to_string();

    assert_eq!(error_message,"No Action with Id 00000000-0000-0000-0000-000000000000");
}

#[test]
fn successful_select_by_index() {
    let single_action_list: im::Vector<Action> = vector!().create_new();

    let test_search_action = single_action_list.select_by_index(0).unwrap();

    assert!(test_search_action == single_action_list[0]);
}

#[test]
fn failed_select_by_index() {
    let single_action_list: im::Vector<Action> = vector!();

    let failed_action_selection = single_action_list.select_by_index(0).unwrap_err();

    assert_eq!(failed_action_selection.to_string() , "No Action at Index 0");
}

#[test]
fn action_print_fail_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let error = format!("{:?}", empty_action_list);
    assert_eq!(error.to_string(), "[]");
}

#[test]
fn action_print_successful_test() {
    let empty_action_list: im::Vector<Action> = vector!();

    let single_action_list = empty_action_list.create_new();


    assert_eq!(format!("{:#?}", single_action_list),
            format!(
"[
    Action {{
        name: \"Default Action\",
        priority: Optional,
        completed: false,
        id: {},
    }},
]",single_action_list[0].get_id().simple()));
    }

#[test]
fn action_print_table_successful() {
    let empty_action_list: im::Vector<Action> = vector!();

    let single_action_list = empty_action_list.create_new();

    let table = Table::new(single_action_list.clone());

    assert_eq!(table.to_string(),format!(
"+----------------+----------+-----------+--------------------------------------+
| name           | priority | completed | id                                   |
+----------------+----------+-----------+--------------------------------------+
| Default Action | Optional | false     | {} |
+----------------+----------+-----------+--------------------------------------+",
        &single_action_list[0].get_id()));
    }

#[test]
#[should_panic]
fn failing_action_removal_test() {
    let empty_action_list: im::Vector<Action> = vector!();

    empty_action_list.remove_action(0).unwrap_err();
}

#[test]
fn successful_action_removal_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let single_action_list = &empty_action_list.create_new();

    let good_result = &single_action_list.remove_action(0).unwrap();

    assert!(good_result.is_empty());
}

#[test]
fn action_completion_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let single_action_list = &empty_action_list.create_new();
    let good_result = &single_action_list.toggle_completion_status(0).unwrap();
    assert_eq!(good_result[0].get_completion_status() , true);
}

#[test]
fn action_reopen() {
    let single_action_list: im::Vector<Action> = vector!().create_new();

    let updated_action_list = &single_action_list
        .toggle_completion_status(0).unwrap()
        .toggle_completion_status(0).unwrap();

    assert_eq!(updated_action_list[0].get_completion_status(), false);
}

#[test]
fn list_member_rename() {
    let single_action_list: im::Vector<Action> = vector!().create_new();

    let good_result = &single_action_list
        .rename(0, "Changed Task".to_string())
        .unwrap();

    assert!(good_result[0].get_name() == "Changed Task".to_string());
}

#[test]
fn action_reprioritize() {
    let empty_action_list: im::Vector<Action> = vector!();
    let single_action_list = &empty_action_list.create_new();
    let changed_action_list = &single_action_list
        .change_priority(0, "low".to_string())
        .unwrap();
    assert_eq!(changed_action_list[0].get_priority(), Priority::Low);
}

#[test]
fn get_name(){
    let empty_action_list: im::Vector<Action> = vector!().create_new();

    let name = &empty_action_list.get_action_name(0).unwrap();
    assert_eq!(name.to_string(), "Default Action");
}

#[test]
fn get_priority(){
    let empty_action_list: im::Vector<Action> = vector!().create_new();

    let priority = &empty_action_list.get_action_priority(0).unwrap();

    assert_eq!(priority.to_string(), "Optional");
}

#[test]
fn get_completion_status(){
    let empty_action_list: im::Vector<Action> = vector!().create_new();

    let completion_status = empty_action_list.get_action_completion_status(0).unwrap();

    assert_eq!(completion_status, false);
}
