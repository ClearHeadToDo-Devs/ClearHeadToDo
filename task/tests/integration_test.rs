use std::error::Error;
use task::*;
use uuid::Uuid;

pub fn create_nil_task() -> Task {
    Task {
        id: Uuid::nil(),
        ..Default::default()
    }
}

#[test]
fn default_task_creation() {
    let test_task = create_nil_task();
    assert!(test_task.name == "Default Task".to_string());
    assert!(test_task.priority == PriEnum::Optional);
    assert!(test_task.completed == false);
    assert!(test_task.id.to_string() == "00000000-0000-0000-0000-000000000000".to_string());
    assert!(test_task.due_date == None);
}

#[test]
fn print_task_content() {
    let test_task = create_nil_task();
    let test_task_string = test_task.export_fields_as_string();
    assert_eq!(
        test_task_string,
        "Default Task,Optional,false,00000000-0000-0000-0000-000000000000\n",
    );
}

#[test]
fn task_creation_unique_id() {
    let first_test_task = Task::create_default_task();
    let second_test_task = Task::create_default_task();

    assert!(first_test_task.id != second_test_task.id);
}

#[test]
fn rename() {
    let test_task = Task::create_default_task();
    let renamed_task = &test_task.rename(&"Changed Name".to_string());

    assert!(renamed_task.name == "Changed Name");
}

#[test]
fn completion() -> Result<(), Box<dyn Error>> {
    let test_task = Task::create_default_task();
    let test_successful_completion_task = &test_task.toggle_completion_status();

    assert!(test_successful_completion_task.completed == true);
    return Ok(());
}

#[test]
fn reopen() -> () {
    let test_task = Task::create_default_task();
    let test_first_completion_task = &test_task.toggle_completion_status();
    let reopened_task = &test_first_completion_task.toggle_completion_status();
    assert_eq!(reopened_task.completed, false);
}

#[test]
fn failing_reprioritize() -> Result<(), Box<dyn Error>> {
    let test_task = Task::create_default_task();
    let error = &test_task.change_priority("6").unwrap_err();
    assert_eq!(error.to_string(), "invalid priority");
    return Ok(());
}

#[test]
fn successful_reprioritize() -> Result<(), Box<dyn Error>> {
    let priority_5_test_task = Task::create_default_task();

    let priority_4_test_task = &priority_5_test_task.change_priority("4")?;
    assert!(priority_4_test_task.priority == priority::PriEnum::Low);

    let priority_3_test_task = &priority_4_test_task.change_priority("3")?;
    assert!(priority_3_test_task.priority == PriEnum::Medium);

    let priority_2_test_task = &priority_3_test_task.change_priority("2")?;
    assert!(priority_2_test_task.priority == PriEnum::High);

    let priority_1_test_task = &priority_2_test_task.change_priority("1")?;
    assert!(priority_1_test_task.priority == PriEnum::Critical);

    return Ok(());
}
