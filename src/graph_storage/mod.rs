use crate::{action::Action, action_interface::ActionViewing, priority::Priority};
use indradb::{Identifier, Json, Vertex, VertexProperties, VertexProperty};
use serde_json::{Number, Value};

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn create_action_vertex() {
        let action_vertex = Vertex::new(Identifier::new("Action").unwrap());

        assert!(action_vertex.t == Identifier::new("Action").unwrap());
    }
}
