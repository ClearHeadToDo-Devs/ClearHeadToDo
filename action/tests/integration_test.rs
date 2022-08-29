use std::error::Error;
use action::*;
use uuid::Uuid;
use im::vector;
use std::str::FromStr;



#[test]
fn task_list_creation() {
    let test_task_list: im::Vector<Action> = vector!();
    assert_eq!(test_task_list, vector!());
}

#[test]
fn child_task_addition() -> Result<(), Box<dyn Error>> {
    let empty_task_list: im::Vector<Action> = vector!();
    let single_task_list = add_nil_action(empty_task_list);
    let test_task = &single_task_list[0];
    assert!(test_task.name == "Default Action");
    assert!(test_task.completed == false);
    assert!(test_task.priority == Priority::Optional);
    assert!(test_task.id == Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
    assert!(&single_task_list[0] == test_task);
    return Ok(());
}

#[test]
fn task_successful_search_by_id_test() -> Result<(), Box<dyn Error>> {
    let empty_task_list: im::Vector<Action> = vector!();
    let single_nil_task_list = add_nil_action(empty_task_list);
    let test_search_task = single_nil_task_list.select_by_id(Uuid::nil());
    assert!(
        test_search_task.unwrap()
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
fn task_failed_search_by_id_test() -> Result<(), Box<dyn Error>> {
    let empty_task_list: im::Vector<Action> = vector!();
    let test_search_task = empty_task_list
        .select_by_id(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
    
    let error_message = test_search_task.unwrap_err().to_string();
    assert_eq!(error_message,"No Action with Id 00000000-0000-0000-0000-000000000000");
    return Ok(());
}

#[test]
fn task_print_fail_test() {
    let empty_task_list: im::Vector<Action> = vector!();
    let error = &empty_task_list.print_list().unwrap_err();
    assert_eq!(error.to_string(), "list is empty");
}

#[test]
fn task_print_successful_test() {
    let empty_task_list: im::Vector<Action> = vector!();
    let single_task_list = add_nil_action(empty_task_list);

    let success = &single_task_list.print_list().unwrap();

    assert_eq!(
                format!("{}", success.to_string()),
                "name,priority,completed,ID\nDefault Action,Optional,false,00000000-0000-0000-0000-000000000000\n"
            );
}

#[test]
fn failing_task_removal_test() {
    let empty_task_list: im::Vector<Action> = vector!();
    let error = &empty_task_list.remove(0).unwrap_err();
    assert_eq!(error.to_string(), "No Action at Index 0");
}

#[test]
fn successful_task_removal_test() {
    let empty_task_list: im::Vector<Action> = vector!();
    let single_task_list = &empty_task_list.create_new();
    let good_result = &single_task_list.remove(0).unwrap();
    assert!(good_result.is_empty());
}

#[test]
fn failing_task_completion_bad_index_test() {
    let empty_task_list: im::Vector<Action> = vector!();
    let error = &empty_task_list
        .toggle_completion_status(0)
        .unwrap_err();
    assert_eq!(error.to_string(), "No Action at Index 0");
}

#[test]
fn successful_task_completion_test() {
    let empty_task_list: im::Vector<Action> = vector!();
    let single_task_list = &empty_task_list.create_new();
    let good_result = &single_task_list.toggle_completion_status(0).unwrap();
    assert!(good_result[0].completed == true);
}

#[test]
fn successful_task_reopen_test() {
    let mut empty_task_list: im::Vector<Action> = vector!();
    empty_task_list.push_front(Action {
        completed: true,
        ..Default::default()
    });
    let updated_task_list = &empty_task_list.toggle_completion_status(0).unwrap();
    assert_eq!(updated_task_list[0].completed, false);
}

#[test]
fn failing_task_rename_bad_index_test() {
    let empty_task_list: im::Vector<Action> = vector!();
    let error = &empty_task_list
        .rename(0, "Change Test".to_string())
        .unwrap_err();
    assert_eq!(error.to_string(), "No Action at Index 0");
}

#[test]
fn successful_task_rename_test() {
    let empty_task_list: im::Vector<Action> = vector!();
    let single_task_list = &empty_task_list.create_new();
    let good_result = &single_task_list
        .rename(0, "Changed Task".to_string())
        .unwrap();
    assert!(good_result[0].name == "Changed Task".to_string());
}

#[test]
fn failing_task_reprioritize_bad_index_test() {
    let empty_task_list: im::Vector<Action> = vector!();
    let error = &empty_task_list
        .change_priority(0, "Optional".to_string())
        .unwrap_err();
    assert_eq!(error.to_string(), "No Action at Index 0");
}

#[test]
fn failing_task_reprioritize_bad_priority_test() {
    let empty_task_list: im::Vector<Action> = vector!();
    let single_task_list = &empty_task_list.create_new();
    let error = &single_task_list
        .change_priority(0, "bad priority".to_string())
        .unwrap_err();
    assert_eq!(error.to_string(), "bad priority is an Invalid Priority Option".to_string());
}

#[test]
fn successful_task_reprioritize_test() {
    let empty_task_list: im::Vector<Action> = vector!();
    let single_task_list = &empty_task_list.create_new();
    let changed_task_list = &single_task_list
        .change_priority(0, "low".to_string())
        .unwrap();
    assert_eq!(changed_task_list[0].priority, Priority::Low);
}
