use crate::relationships::test_utilities;
use crate::relationships::EdgeDirectionality;
use crate::test_utilities::test_utilities::create_nil_relationship;
use crate::Relationship;
use crate::RelationshipVariant;
use crate::Uuid;
use im::Vector;
pub fn add_nil_relationship_to_vector(list: Vector<Relationship>) -> Vector<Relationship> {
    let nil_relationship = create_nil_relationship(
        RelationshipVariant::Related(EdgeDirectionality::Undirected),
        Uuid::nil(),
        Uuid::nil(),
    );
    let mut cloned_list = list.clone();

    cloned_list.push_back(nil_relationship);

    return cloned_list;
}
