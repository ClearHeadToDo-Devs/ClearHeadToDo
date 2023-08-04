use indradb::{PipePropertyQuery, QueryExt, SpecificVertexQuery};
use uuid::Uuid;

pub fn create_single_propertied_vertex_query(id: Uuid) -> PipePropertyQuery {
    create_single_vertex_query(id).properties().unwrap()
}

pub fn create_single_vertex_query(id: Uuid) -> SpecificVertexQuery {
    SpecificVertexQuery::single(id)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_sample_vertex_query() {
        let vertex_id = Uuid::nil();

        let vertex_query = create_single_vertex_query(vertex_id);

        assert!(vertex_query == SpecificVertexQuery::single(Uuid::nil()))
    }

    #[test]
    fn create_sample_propertied_vertex_query() {
        let vertex_id = Uuid::nil();

        let vertex_query = create_single_propertied_vertex_query(vertex_id);

        assert!(
            vertex_query
                == PipePropertyQuery::new(Box::new(
                    SpecificVertexQuery::single(Uuid::nil()).into()
                ))
                .unwrap()
        )
    }
}
