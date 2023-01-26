use anyhow::Result;
use indradb::{
    Datastore, EdgeKey, PropertyValueVertexQuery, SpecificVertexQuery, Vertex, VertexQueryExt,
};
use uuid::Uuid;

use crate::data::types::DatastoreType;

use super::{
    node_data_identifier, node_edge_identifier, node_identifier, node_name_identifier, Node,
    NodeWithEdges,
};

pub fn create_node(datastore: &DatastoreType, data: Node) -> Result<Vertex> {
    let v = Vertex::with_id(data.id, node_identifier());

    let q_data = SpecificVertexQuery::single(v.id.clone()).property(node_data_identifier());
    let q_name = SpecificVertexQuery::single(v.id.clone()).property(node_name_identifier());

    datastore.create_vertex(&v)?;
    datastore.set_vertex_properties(q_data, serde_json::to_value(data.clone())?)?;
    datastore.set_vertex_properties(q_name, serde_json::to_value(data.name)?)?;

    Ok(v)
}

pub fn create_node_with_parent(
    datastore: &DatastoreType,
    data: Node,
    parent_id: Uuid,
) -> Result<NodeWithEdges> {
    let vertex = create_node(datastore, data)?;

    get_node_by_id(datastore, parent_id.clone())
        .map_err(|_| anyhow::anyhow!("Parent node not found"))?;

    let k = EdgeKey::new(parent_id, node_edge_identifier(), vertex.id);
    datastore.create_edge(&k)?;

    let node_with_edges = get_node_by_id(datastore, vertex.id)?;

    Ok(node_with_edges)
}

pub fn update_node(datastore: &DatastoreType, data: Node) -> Result<NodeWithEdges> {
    let q = SpecificVertexQuery::single(data.id).property(node_data_identifier());

    datastore.set_vertex_properties(q, serde_json::to_value(data.clone())?)?;

    let node_with_edges = get_node_by_id(datastore, data.id)?;

    Ok(node_with_edges)
}

pub fn delete_node(datastore: &DatastoreType, id: Uuid) -> Result<bool> {
    let q = SpecificVertexQuery::single(id);

    let vertexes = datastore.get_vertices(q.clone().into())?;

    if vertexes.is_empty() {
        return Ok(false);
    }

    datastore.delete_vertices(q.clone().into())?;
    datastore.delete_vertex_properties(q.clone().property(node_data_identifier()).into())?;
    datastore.delete_edges(q.clone().inbound().into())?;
    datastore.delete_edges(q.clone().outbound().into())?;

    Ok(true)
}

pub fn get_node_by_id(datastore: &DatastoreType, id: Uuid) -> Result<NodeWithEdges> {
    let q = SpecificVertexQuery::single(id);

    let inbound_edges = datastore.get_edges(q.clone().inbound().into())?;
    let outbound_edges = datastore.get_edges(q.clone().outbound().into())?;

    let props = datastore.get_all_vertex_properties(q.clone().into())?;

    let node = props
        .iter()
        .fold(None, |_: Option<Node>, p| {
            let prop = p.props.iter().find(|p| p.name == node_data_identifier());

            prop.map(|p| serde_json::from_value(p.value.clone()).unwrap())
        })
        .ok_or(anyhow::anyhow!("Node data not found"))?;

    let parents = inbound_edges
        .iter()
        .map(|e| e.key.outbound_id)
        .collect::<Vec<Uuid>>();

    let children = outbound_edges
        .iter()
        .map(|e| e.key.inbound_id)
        .collect::<Vec<Uuid>>();

    let node_with_edges = NodeWithEdges::new(node, parents, children);

    Ok(node_with_edges)
}

pub fn get_node_by_name(datastore: &DatastoreType, name: String) -> Result<NodeWithEdges> {
    let q = PropertyValueVertexQuery::new(node_name_identifier(), serde_json::json!(name));

    let binding = datastore.get_all_vertex_properties(q.into())?;
    let node = binding
        .iter()
        .fold(None, |_: Option<Node>, p| {
            let prop = p.props.iter().find(|p| p.name == node_data_identifier());

            prop.map(|p| serde_json::from_value(p.value.clone()).unwrap())
        })
        .ok_or(anyhow::anyhow!("Node not found"))?;

    let node_with_edges = get_node_by_id(datastore, node.id)?;

    Ok(node_with_edges)
}
