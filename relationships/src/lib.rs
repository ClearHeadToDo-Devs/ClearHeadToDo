pub mod relationships;
pub use crate::relationships::Relationship;
pub use crate::relationships::RelationshipManagement;

use uuid::Uuid;

use im::Vector;

trait RelationshipListManagement {
    type L: RelationshipListManagement;
    fn add_related(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L;
    fn add_sequential(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L;
    fn add_parental(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L;
    fn remove_at_index(&self, index: usize) -> Self::L;
}

impl RelationshipListManagement for Vector<Relationship> {
    type L = Vector<Relationship>;

    fn add_related(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let new_relationship = Relationship::create_new_related(participant_1, participant_2);
        let mut cloned_list = self.clone();

        cloned_list.push_back(new_relationship);

        return cloned_list;
    }
    fn add_sequential(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let new_relationship = Relationship::create_new_sequential(participant_1, participant_2);
        let mut cloned_list = self.clone();

        cloned_list.push_back(new_relationship);

        return cloned_list;
    }

    fn add_parental(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let new_relationship = Relationship::create_new_parental(participant_1, participant_2);
        let mut cloned_list = self.clone();

        cloned_list.push_back(new_relationship);

        return cloned_list;
    }

    fn remove_at_index(&self, index: usize) -> Self::L {
        let mut updated_list = self.clone();

        if index == 0 {
            updated_list.pop_front();
            return updated_list;
        } else if index == updated_list.len() - 1 {
            updated_list.pop_back();
            return updated_list;
        } else {
            updated_list.remove(index);
            return updated_list;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_related_relationship_to_list() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let modified_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        assert! {modified_list[0].get_variant_string() == "Related: Undirected"}
    }

    #[test]
    fn add_sequential_relationship_to_list() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let modified_list = relationship_list.add_sequential(Uuid::nil(), Uuid::nil());

        assert! {modified_list[0].get_variant_string() == "Sequential: Directed"}
    }

    #[test]
    fn add_parental_relationship_to_list() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let modified_list = relationship_list.add_parental(Uuid::nil(), Uuid::nil());

        assert!(modified_list[0].get_variant_string() == "Parental: Directed")
    }

    #[test]
    fn remove_relationship() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let modified_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        let pruned_list = modified_list.remove_at_index(0);

        assert!(pruned_list.len() == 0);
    }

    #[test]
    fn remove_first_relationship() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let modified_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());
        let second_modified_list = modified_list.add_related(Uuid::nil(), Uuid::nil());

        let popped_list = second_modified_list.remove_at_index(1);

        assert!(popped_list[0].get_variant_string() == "Related: Undirected");
    }

    #[test]
    fn remove_middle_relationship() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let modified_list = relationship_list.add_sequential(Uuid::nil(), Uuid::nil());
        let second_modified_list = modified_list.add_related(Uuid::nil(), Uuid::nil());
        let third_modified_list = second_modified_list.add_parental(Uuid::nil(), Uuid::nil());

        let popped_list = third_modified_list.remove_at_index(1);

        assert!(
            popped_list[1].get_variant_string() == "Parental: Directed" && popped_list.len() == 2
        );
    }

    #[test]
    fn out_of_bounds_removal_error() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let failed_poped_list = relationship_list.remove_at_index(0);

        //assert!(failed_poped_list == "out of bounds index")
        // don't really know what is going on here...
    }
}
