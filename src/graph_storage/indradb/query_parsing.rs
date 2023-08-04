use std::{error::Error, fmt};

use indradb::{Database, Edge, MemoryDatastore, Query, QueryOutputValue, Vertex, VertexProperties};

use crate::graph_storage::db_error::GraphDatabaseError;
pub fn parse_single_output_query(
    database: &Database<MemoryDatastore>,
    query: Query,
) -> Result<QueryOutputValue, Box<dyn Error>> {
    let binding = database.get(query)?;

    let query_output = binding.first().ok_or(GraphDatabaseError::NoOutput)?;

    Ok(query_output.clone())
}
pub fn parse_vertices(query_output: QueryOutputValue) -> Result<Vec<Vertex>, Box<dyn Error>> {
    if let QueryOutputValue::Vertices(vertex_list) = query_output {
        if vertex_list.is_empty() {
            return Err(Box::new(QueryParseError::EmptyOutput));
        }
        Ok(vertex_list)
    } else {
        Err(Box::new(QueryParseError::InvalidOutput))
    }
}
pub fn parse_propertied_vertices(
    query_output: QueryOutputValue,
) -> Result<Vec<VertexProperties>, Box<dyn Error>> {
    if let QueryOutputValue::VertexProperties(vertex_list) = query_output {
        if vertex_list.is_empty() {
            return Err(Box::new(QueryParseError::EmptyOutput));
        }
        Ok(vertex_list)
    } else {
        Err(Box::new(QueryParseError::InvalidOutput))
    }
}
pub fn parse_edges(query_output: QueryOutputValue) -> Result<Vec<Edge>, Box<dyn Error>> {
    if let QueryOutputValue::Edges(edge_list) = query_output {
        if edge_list.is_empty() {
            return Err(Box::new(QueryParseError::EmptyOutput));
        }
        Ok(edge_list)
    } else {
        Err(Box::new(QueryParseError::InvalidOutput))
    }
}

#[derive(Debug, Clone)]
enum QueryParseError {
    InvalidOutput,
    EmptyOutput,
}

impl fmt::Display for QueryParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QueryParseError::InvalidOutput => write!(
                f,
                "This is not the type of output expected, cannot convert to desired output type"
            ),
            QueryParseError::EmptyOutput => write!(f, "Empty Output"),
        }
    }
}

impl Error for QueryParseError {}

#[cfg(test)]
mod tests {
    use indradb::AllVertexQuery;
    use indradb::{Edge, Json, NamedProperty};

    use super::*;
    #[test]
    fn parse_example_output_query() {
        let database = MemoryDatastore::new_db();

        let test_query: Query = AllVertexQuery.into();

        let query_output = parse_single_output_query(&database, test_query);

        assert!(query_output.is_ok())
    }
    mod output_parsing {
        use super::*;
        mod successful {
            use super::*;
            use indradb::Identifier;
            use serde_json::Value;
            use uuid::Uuid;
            #[test]
            fn parse_single_vertex() {
                let test_output =
                    QueryOutputValue::Vertices(vec![Vertex::new(Identifier::new("test").unwrap())]);

                let vertex_list = parse_vertices(test_output).unwrap();

                assert!(vertex_list[0].t.to_string() == "test")
            }
            #[test]
            fn parse_propertied_vertex() {
                let propertied_vertex = VertexProperties::new(
                    Vertex::new(Identifier::new("test").unwrap()),
                    vec![NamedProperty::new(
                        Identifier::new("test").unwrap(),
                        Json::from(Value::Bool(true)),
                    )],
                );
                let test_output =
                    QueryOutputValue::VertexProperties(vec![propertied_vertex.clone()]);

                let vertex_list = parse_propertied_vertices(test_output).unwrap();

                assert!(vertex_list[0] == propertied_vertex)
            }

            #[test]
            fn parse_single_edge() {
                let edge = Edge::new(Uuid::nil(), Identifier::new("test").unwrap(), Uuid::nil());

                let test_output = QueryOutputValue::Edges(vec![edge.clone()]);

                let edge_list = parse_edges(test_output).unwrap();

                assert!(edge_list[0] == edge)
            }
        }
        mod failed {
            use super::*;
            #[test]
            fn invalid_output_parse_error() {
                let test_output = QueryOutputValue::Count(0);

                let failed_parse = parse_vertices(test_output).unwrap_err();

                assert!(
            failed_parse.to_string()
                == "This is not the type of output expected, cannot convert to desired output type"
        )
            }

            #[test]
            fn parse_empty_output_error() {
                let test_output = QueryOutputValue::Vertices(vec![]);

                let failed_parse = parse_vertices(test_output).unwrap_err();

                assert!(failed_parse.to_string() == "Empty Output")
            }
        }
    }
}
