use std::error::Error;

use std::fmt;

#[derive(Debug, Clone)]
pub enum GraphDatabaseError {
    InvalidUuid,
    InvalidProperty,
    NoPropertiesDefined,
    NoOutgoingEdges,
    NoIncomingEdges,
    NoOutput,
}

impl fmt::Display for GraphDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphDatabaseError::InvalidUuid => write!(f, "No Vertex with this ID"),
            GraphDatabaseError::InvalidProperty => write!(f, "No Property with this name"),
            GraphDatabaseError::NoPropertiesDefined => {
                write!(f, "No Properties Defined for This Vertex")
            }
            GraphDatabaseError::NoOutgoingEdges => {
                write!(f, "No Outbound connections on this vertex")
            }
            GraphDatabaseError::NoIncomingEdges => {
                write!(f, "No Incoming connections on this vertex")
            }
            GraphDatabaseError::NoOutput => {
                write!(f, "No Output from this query")
            }
        }
    }
}

impl Error for GraphDatabaseError {}
