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
        updated_list.remove(index);
        return updated_list;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_related_relationship_to_list() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let modified_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        assert! {modified_list[0].get_variant().to_string() == "Related: Undirected"}
    }

    #[test]
    fn add_sequential_relationship_to_list() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let modified_list = relationship_list.add_sequential(Uuid::nil(), Uuid::nil());

        assert! {modified_list[0].get_variant().to_string() == "Sequential: Directed"}
    }

    #[test]
    fn add_parental_relationship_to_list() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let modified_list = relationship_list.add_parental(Uuid::nil(), Uuid::nil());

        assert!(modified_list[0].get_variant().to_string() == "Parental: Directed")
    }

    #[test]
    fn remove_relationship() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let modified_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        let pruned_list = modified_list.remove_at_index(0);

        assert!(pruned_list.len() == 0);
    }

    #[test]
    #[should_panic]
    fn empty_vector_removal() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let failed_poped_list = relationship_list.remove_at_index(0);

        assert!(failed_poped_list.len() == 0)
    }

    #[test]
    fn return_index_from_id() {
        let relationship_list: Vector<Relationship> =
            Vector::new().add_related(Uuid::nil(), Uuid::nil());
    }
}
