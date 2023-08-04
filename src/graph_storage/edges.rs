use uuid::Uuid;

pub trait EdgeCreation {
    fn append_edge(&self, outbound_id: Uuid, t: &str, inbound_id: Uuid) -> bool;
}

#[derive(PartialEq, Clone)]
pub struct GeneralEdge {
    outbound_id: Uuid,
    edge_type: String,
    inbound_id: Uuid,
}

impl GeneralEdge {
    pub fn new(outbound_id: Uuid, edge_type: String, inbound_id: Uuid) -> Self {
        Self {
            outbound_id,
            edge_type,
            inbound_id,
        }
    }

    pub fn set_outbound_id(&mut self, outbound_id: Uuid) {
        self.outbound_id = outbound_id;
    }
    pub fn get_outbound_id(&self) -> Uuid {
        self.outbound_id
    }

    pub fn set_type(&mut self, edge_type: String) {
        self.edge_type = edge_type;
    }
    pub fn get_type(&self) -> String {
        self.edge_type.clone()
    }

    pub fn set_inbound_id(&mut self, inbound_id: Uuid) {
        self.inbound_id = inbound_id;
    }
    pub fn get_inbound_id(&self) -> Uuid {
        self.inbound_id
    }
}

impl Default for GeneralEdge {
    fn default() -> Self {
        GeneralEdge::new(Uuid::nil(), "".to_string(), Uuid::nil())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> GeneralEdge {
        GeneralEdge::default()
    }

    fn edge_has_default_values(edge: GeneralEdge)-> bool {
        let validator = EdgeValidator::new(edge.clone(), EdgeDirection::Inbound, true);
        
        validator.edge_id_is_nil() &&
        edge.get_type().is_empty() &&
        validator.reverse_direction().edge_id_is_nil()
    
    }

    fn outbound_id_not_nil(edge: GeneralEdge)->bool{
        !EdgeValidator::new(edge, EdgeDirection::Outbound, true).edge_id_is_nil()
    

    }

    fn inbound_id_not_nil(edge: GeneralEdge)->bool{
        !EdgeValidator::new(edge, EdgeDirection::Inbound, true).edge_id_is_nil()
    }

    struct EdgeValidator {
        edge: GeneralEdge,
        direction: EdgeDirection,
        empty: bool,
    }

    impl EdgeValidator {
        fn new(edge: GeneralEdge, direction: EdgeDirection, empty: bool) -> Self {
            Self {
                edge,
                direction,
                empty,
            }
        }

        fn edge_id_is_nil(&self) -> bool {
            match self.direction {
                EdgeDirection::Inbound => match self.empty {
                    true => self.edge.get_inbound_id().is_nil(),
                    false => !self.edge.get_inbound_id().is_nil(),
                }
                EdgeDirection::Outbound => match self.empty {
                    true => self.edge.get_outbound_id().is_nil(),
                    false => !self.edge.get_outbound_id().is_nil(),

                }
            }
        }

        fn reverse_direction(&self)->Self{
            match self.direction{
                EdgeDirection::Inbound => Self::new(self.edge.clone(), EdgeDirection::Outbound, self.empty),
                EdgeDirection::Outbound => Self::new(self.edge.clone(), EdgeDirection::Inbound, self.empty)
            }
        }
    }

    fn edge_is_test_type(edge: GeneralEdge) -> bool {
        edge.get_type() == "test"
    }

    enum EdgeDirection {
        Inbound,
        Outbound,
    }

    #[test]
    fn create_default_edge(){
        let new_edge = setup();

        assert!(edge_has_default_values(new_edge));
    }

    #[test]
    fn update_existing_edge_outbound_id(){
        let mut edge = setup();

        edge.set_outbound_id(Uuid::new_v4());

        assert!(outbound_id_not_nil(edge))
    }

    #[test]
    fn update_existing_edge_type(){
        let mut edge = setup();

        edge.set_type("test".to_string());

        assert!(edge_is_test_type(edge))
    }

    #[test]
    fn update_existing_edge_inbound_id(){
        let mut edge = setup();

        edge.set_inbound_id(Uuid::new_v4());

        assert!(inbound_id_not_nil(edge))
    }
}
