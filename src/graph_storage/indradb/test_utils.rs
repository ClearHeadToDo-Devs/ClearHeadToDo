use indradb::QueryExt;
use indradb::QueryOutputValue;
use uuid::Uuid;

use indradb::{
    Database, Edge, Identifier, Json, MemoryDatastore, SpecificVertexQuery, VertexProperties,
};

#[allow(dead_code)]
pub fn create_database_with_multiple_propertied_vertices() -> (Database<MemoryDatastore>, Uuid, Uuid)
{
    let (database, vertices) = create_database_with_multiple_vertices();

    add_property_to_vertex(&database, vertices[0], "test_property_1");
    add_property_to_vertex(&database, vertices[1], "test_property_2");

    (database, vertices[0], vertices[1])
}

#[allow(dead_code)]
pub fn create_database_with_multi_propertied_vertex() -> (Database<MemoryDatastore>, Uuid) {
    let (database, vertex_id) = create_database_with_propertied_vertex();

    add_property_to_vertex(&database, vertex_id, "test_property_1");

    (database, vertex_id)
}

pub fn create_database_with_propertied_vertex() -> (Database<MemoryDatastore>, Uuid) {
    let (database, vertex_id) = create_database_with_test_vertex();

    add_property_to_vertex(&database, vertex_id, "test_property");

    (database, vertex_id)
}

#[allow(dead_code)]
pub fn create_database_with_connected_vertices() -> (Database<MemoryDatastore>, Uuid, Uuid) {
    let (database, vertices) = create_database_with_multiple_vertices();

    database
        .create_edge(&Edge::new(
            vertices[0],
            create_identifier("test"),
            vertices[1],
        ))
        .unwrap();

    (database, vertices[0], vertices[1])
}

pub fn create_database_with_multiple_vertices() -> (Database<MemoryDatastore>, Vec<Uuid>) {
    let (database, vertex_id) = create_database_with_test_vertex();

    let second_vertex_id = add_vertex_from_type_to_database(&database, "test");

    (database, vec![vertex_id, second_vertex_id])
}

pub fn create_database_with_test_vertex() -> (Database<MemoryDatastore>, Uuid) {
    let database = MemoryDatastore::new_db();

    let vertex_id = add_vertex_from_type_to_database(&database, "test");

    (database, vertex_id)
}

pub fn add_vertex_from_type_to_database(db: &Database<MemoryDatastore>, string: &str) -> Uuid {
    db.create_vertex_from_type(create_identifier(string))
        .unwrap()
}

pub fn add_property_to_vertex(db: &Database<MemoryDatastore>, id: Uuid, property_name: &str) {
    db.set_properties(
        SpecificVertexQuery::single(id),
        create_identifier(property_name),
        &Json::new(true.into()),
    )
    .unwrap();
}

#[allow(dead_code)]
pub fn get_property_for_vertex(db: &Database<MemoryDatastore>, id: Uuid) -> VertexProperties {
    let query_vertex: QueryOutputValue = db
        .get(SpecificVertexQuery::single(id).properties().unwrap())
        .unwrap()
        .first()
        .unwrap()
        .clone();

    match query_vertex {
        QueryOutputValue::VertexProperties(properties) => properties[0].clone(),
        _ => panic!("Expected vertex"),
    }
}

pub fn create_identifier(name: &str) -> Identifier {
    Identifier::new(name).unwrap()
}
