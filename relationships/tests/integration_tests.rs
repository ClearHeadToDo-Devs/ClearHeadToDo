use relationships::item::edge_direction::EdgeDirectionality;

use uuid::Uuid;
use im::Vector;
use relationships::item::Relationship;
use relationships::item::RelationshipVariant;
use relationships::RelationshipListManagement;


pub fn invalid_index_error_string() -> String {
    return String::from("Unable to find Relationship at given Index");
}

pub fn create_relationship_list_with_single_related_relationship() -> Vector<Relationship> {
    let mut list = Vector::new();
    let relationship = Relationship::create_new_related(
        Uuid::nil(),
        Uuid::nil(),
    );

    list.push_back(relationship);

    return list;
}

pub fn create_relationship_list_with_single_relationship(variant: &str) -> Vector<Relationship> {
    let mut list: Vector<Relationship> = Vector::new();

    let relationship = Relationship::create_new(variant,Uuid::nil(),Uuid::nil()).unwrap();

    list.push_back(relationship);

    return list;
}

#[test]
fn create_new_from_string() {
    let relationship_list: Vector<Relationship> = Vector::new();
    let variant_string = "related".to_string();

    let updated_list = relationship_list
        .add_new(&variant_string, Uuid::nil(), Uuid::nil())
        .unwrap();

    assert_eq!(updated_list[0].get_variant() , RelationshipVariant::create_related());
}

#[test]
fn create_sequential_from_string() {
    let relationship_list: Vector<Relationship> = Vector::new();
    let variant_string = "sequential".to_string();

    let updated_list = relationship_list
        .add_new(&variant_string, Uuid::nil(), Uuid::nil())
        .unwrap();

    assert_eq!(updated_list[0].get_variant() , RelationshipVariant::create_sequential());
}

#[test]
fn create_parental_from_string() {
    let relationship_list: Vector<Relationship> = Vector::new();
    let variant_string = "parental".to_string();

    let updated_list = relationship_list
        .add_new(&variant_string, Uuid::nil(), Uuid::nil())
        .unwrap();

    assert_eq!(updated_list[0].get_variant() , RelationshipVariant::create_parental());
}

#[test]
fn failed_create_from_string() {
    let relationship_list: Vector<Relationship> = Vector::new();
    let variant_string = "invalid".to_string();

    let updated_list = relationship_list
        .add_new(&variant_string, Uuid::nil(), Uuid::nil());

    assert!(updated_list.is_err());
}

#[test]
fn add_related_to_list() {
    let relationship_list: Vector<Relationship> = Vector::new();

    let modified_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

    assert_eq!(modified_list[0].get_variant() , RelationshipVariant::create_related());
}

#[test]
fn add_sequential_to_list() {
    let relationship_list: Vector<Relationship> = Vector::new();

    let modified_list = relationship_list.add_sequential(Uuid::nil(), Uuid::nil());

    assert_eq!(modified_list[0].get_variant(),RelationshipVariant::create_sequential());
}

#[test]
fn add_parental_to_list() {
    let relationship_list: Vector<Relationship> = Vector::new();

    let modified_list = relationship_list.add_parental(Uuid::nil(), Uuid::nil());

    assert_eq!(modified_list[0].get_variant() , RelationshipVariant::create_parental());
}

#[test]
fn remove_relationship() {
    let relationship_list = create_relationship_list_with_single_related_relationship();

    let pruned_list = relationship_list.remove_at_index(0).unwrap();

    assert_eq!(pruned_list.len() , 0);
}

#[test]
fn empty_vector_removal_error() {
    let relationship_list: Vector<Relationship> = Vector::new();

    let failed_poped_list = relationship_list.remove_at_index(0).unwrap_err();

    assert_eq!(failed_poped_list.to_string() , invalid_index_error_string());
}

#[test]
fn return_relationship_from_id() {
    let relationship_list: Vector<Relationship> =
        create_relationship_list_with_single_related_relationship();

    let relationship_id = relationship_list
        .select_by_id(relationship_list[0].get_id())
        .unwrap();

    assert!(relationship_id == relationship_list[0]);
}

#[test]
fn fail_to_find_id() {
    let relationship_list: Vector<Relationship> = Vector::new();

    let failed_id_query = relationship_list
        .select_by_id(Uuid::nil())
        .unwrap_err();

    assert!(failed_id_query == "cannot find this id within the relationship list".to_string())
}

#[test]
fn successfully_get_id() {
    let relationship_list = create_relationship_list_with_single_related_relationship();

    let relationship = relationship_list.select_by_index(0).unwrap();

    assert_eq!(relationship, relationship_list[0]);
}

#[test]
fn failed_get_id() {
    let test_list: Vector<Relationship> = Vector::new();

    let extraction_error = test_list.select_by_index(0).unwrap_err();

   assert_eq!(extraction_error.to_string() , invalid_index_error_string());
}

#[test]
fn successfully_get_variant() {
    let relationship_list = create_relationship_list_with_single_related_relationship();

    let variant = relationship_list.get_variant(0).unwrap();

    assert!(variant == RelationshipVariant::create_related())
}

#[test]
fn failed_get_variant() {
    let test_list: Vector<Relationship> = Vector::new();

    let index_error = test_list.get_variant(0).unwrap_err();

    assert_eq!(index_error.to_string() , invalid_index_error_string());
}

#[test]
fn successfully_get_participant_1() {
    let test_list: Vector<Relationship> = create_relationship_list_with_single_related_relationship();

    let participant_1 = test_list
        .get_participant_1(0)
        .unwrap();

    assert_eq!(participant_1, Uuid::nil());
}

#[test]
fn failed_get_participant_1() {
    let test_list: Vector<Relationship> = Vector::new();

    let id_error = test_list.get_participant_1(0).unwrap_err();

    assert_eq!(id_error.to_string() , invalid_index_error_string());
}

#[test]
fn successfully_get_participant_2() {
    let test_list: Vector<Relationship> = Vector::new();
    let single_relationship_list = test_list.add_parental(Uuid::nil(), Uuid::nil());

    let participant_2 = single_relationship_list
        .get_participant_2(0)
        .unwrap();

    assert!(participant_2 == single_relationship_list[0].get_participant_2())
}

#[test]
fn failed_get_participant_2() {
    let test_list: Vector<Relationship> = Vector::new();

    let empty_list_error = test_list.get_participant_2(0).unwrap_err();

    assert_eq!(empty_list_error.to_string() , invalid_index_error_string());
}

#[test]
fn successfully_remove_from_id() {
    let relationship_list: Vector<Relationship> = Vector::new();
    let single_relationship_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

    let empty_list = single_relationship_list
        .remove_with_id(single_relationship_list[0].get_id())
        .unwrap();

    assert!(empty_list.len() == 0)
}

#[test]
fn failed_remove_from_id() {
    let relationship_list: Vector<Relationship> = Vector::new();

    let failed_id_query = relationship_list.remove_with_id(Uuid::nil()).unwrap_err();

    assert!(failed_id_query.to_string() == "cannot find this id within the relationship list".to_string())
}

#[test]
fn change_related_to_parental() {
    let relationship_list: Vector<Relationship> = Vector::new();
    let test_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

    let updated_list = test_list.change_variant(0, "parental").unwrap();

    assert!(
        updated_list[0].get_variant()
            == RelationshipVariant::Parental(EdgeDirectionality::Directed)
    );
}

#[test]
fn change_parental_to_related() {
    let relationship_list: Vector<Relationship> = Vector::new();
    let test_list = relationship_list.add_parental(Uuid::nil(), Uuid::nil());

    let updated_list = test_list.change_variant(0, "related").unwrap();

    assert!(
        updated_list[0].get_variant()
            == RelationshipVariant::Related(EdgeDirectionality::Undirected)
    )
}

#[test]
fn failed_change_variant() {
    let relationship_list: Vector<Relationship> = Vector::new();
    let test_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

    let failed_output = test_list.change_variant(0, "bad variant").unwrap_err();

    assert!(failed_output.to_string() == "invalid relationship variant")
}

#[test]
fn update_participant_1_id() {
    let relationship_list: Vector<Relationship> = Vector::new();
    let test_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

    let updated_list = test_list
        .update_participant_1(0, Uuid::new_v4())
        .unwrap();

    assert!(updated_list[0].get_participant_1() != Uuid::nil())
}

#[test]
fn failed_id_update_participant_1() {
    let relationship_list: Vector<Relationship> = Vector::new();

    let bad_list_search = relationship_list
        .update_participant_1(0, Uuid::new_v4())
        .unwrap_err();

    assert!(bad_list_search.to_string() == invalid_index_error_string())
}

#[test]
fn update_participant_2_id() {
    let relationship_list: Vector<Relationship> = Vector::new();
    let test_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

    let updated_list = test_list
        .update_participant_2(0, Uuid::new_v4())
        .unwrap();

    assert!(updated_list[0].get_participant_2() != Uuid::nil())
}

#[test]
fn failed_id_update_participant_2() {
    let relationship_list: Vector<Relationship> = Vector::new();

    let bad_list_search = relationship_list
        .update_participant_2(0, Uuid::new_v4())
        .unwrap_err();

    assert!(bad_list_search.to_string() == invalid_index_error_string())
}
