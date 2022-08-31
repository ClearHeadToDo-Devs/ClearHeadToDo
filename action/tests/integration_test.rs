use std::error::Error;
use action::*;
use uuid::Uuid;
use im::vector;
use std::str::FromStr;



#[test]
fn action_list_creation() {
    let test_action_list: im::Vector<Action> = vector!();
    assert_eq!(test_action_list, vector!());
}

#[test]
fn child_action_addition() -> Result<(), Box<dyn Error>> {
    let empty_action_list: im::Vector<Action> = vector!();
    let single_action_list = add_nil_action(empty_action_list);
    let test_action = &single_action_list[0];
    assert!(test_action.name == "Default Action");
    assert!(test_action.completed == false);
    assert!(test_action.priority == Priority::Optional);
    assert!(test_action.id == Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
    assert!(&single_action_list[0] == test_action);
    return Ok(());
}

#[test]
fn action_successful_search_by_id_test() -> Result<(), Box<dyn Error>> {
    let empty_action_list: im::Vector<Action> = vector!();
    let single_nil_action_list = add_nil_action(empty_action_list);
    let test_search_action = single_nil_action_list.select_by_id(Uuid::nil());
    assert!(
        test_search_action.unwrap()
            == Action {
                id: Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
                name: String::from("Default Action"),
                completed: false,
                priority: Priority::Optional,
            }
    );

    return Ok(());
}

#[test]
fn action_failed_search_by_id_test() -> Result<(), Box<dyn Error>> {
    let empty_action_list: im::Vector<Action> = vector!();
    let test_search_action = empty_action_list
        .select_by_id(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
    
    let error_message = test_search_action.unwrap_err().to_string();
    assert_eq!(error_message,"No Action with Id 00000000-0000-0000-0000-000000000000");
    return Ok(());
}

#[test]
fn action_print_fail_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let error = &empty_action_list.get_list().unwrap_err();
    assert_eq!(error.to_string(), "list is empty");
}

#[test]
fn action_print_successful_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let single_action_list = add_nil_action(empty_action_list);

    let success = &single_action_list.get_list().unwrap();

    assert_eq!(
                format!("{}", success.to_string()),
                "order,name,priority,completed,ID\n0,Default Action,Optional,false,00000000-0000-0000-0000-000000000000"
            );
}

#[test]
fn failing_action_removal_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let error = &empty_action_list.remove(0).unwrap_err();
    assert_eq!(error.to_string(), "No Action at Index 0");
}

#[test]
fn successful_action_removal_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let single_action_list = &empty_action_list.create_new();
    let good_result = &single_action_list.remove(0).unwrap();
    assert!(good_result.is_empty());
}

#[test]
fn failing_action_completion_bad_index_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let error = &empty_action_list
        .toggle_completion_status(0)
        .unwrap_err();
    assert_eq!(error.to_string(), "No Action at Index 0");
}

#[test]
fn successful_action_completion_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let single_action_list = &empty_action_list.create_new();
    let good_result = &single_action_list.toggle_completion_status(0).unwrap();
    assert!(good_result[0].completed == true);
}

#[test]
fn successful_action_reopen_test() {
    let mut empty_action_list: im::Vector<Action> = vector!();
    empty_action_list.push_front(Action {
        completed: true,
        ..Default::default()
    });
    let updated_action_list = &empty_action_list.toggle_completion_status(0).unwrap();
    assert_eq!(updated_action_list[0].completed, false);
}

#[test]
fn failing_action_rename_bad_index_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let error = &empty_action_list
        .rename(0, "Change Test".to_string())
        .unwrap_err();
    assert_eq!(error.to_string(), "No Action at Index 0");
}

#[test]
fn successful_action_rename_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let single_action_list = &empty_action_list.create_new();
    let good_result = &single_action_list
        .rename(0, "Changed Task".to_string())
        .unwrap();
    assert!(good_result[0].name == "Changed Task".to_string());
}

#[test]
fn failing_action_reprioritize_bad_index_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let error = &empty_action_list
        .change_priority(0, "Optional".to_string())
        .unwrap_err();
    assert_eq!(error.to_string(), "No Action at Index 0");
}

#[test]
fn failing_action_reprioritize_bad_priority_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let single_action_list = &empty_action_list.create_new();
    let error = &single_action_list
        .change_priority(0, "bad priority".to_string())
        .unwrap_err();
    assert_eq!(error.to_string(), "bad priority is an Invalid Priority Option".to_string());
}

#[test]
fn successful_action_reprioritize_test() {
    let empty_action_list: im::Vector<Action> = vector!();
    let single_action_list = &empty_action_list.create_new();
    let changed_action_list = &single_action_list
        .change_priority(0, "low".to_string())
        .unwrap();
    assert_eq!(changed_action_list[0].priority, Priority::Low);
}

#[test]
fn successfully_get_id_from_index(){
    let empty_action_list: im::Vector<Action> = vector!();
    let mut single_action_list = empty_action_list.create_new();
    single_action_list[0].id = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
    let id = &single_action_list.get_id_by_index(0).unwrap();
    assert_eq!(id.to_string(), "00000000-0000-0000-0000-000000000000");
}
