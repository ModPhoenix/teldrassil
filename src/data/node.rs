use anyhow::Result;
use chrono::{DateTime, Utc};
use indradb::{Datastore, EdgeKey, Identifier, SpecificVertexQuery, Vertex, VertexQueryExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::DatastoreType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeData {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub data: NodeData,
    pub parents: Vec<Uuid>,
    pub children: Vec<Uuid>,
}

const NODE_IDENTIFIER: &str = "node";
const NODE_DATA_IDENTIFIER: &str = "node_data";
const NODE_LINK_IDENTIFIER: &str = "node_link";

pub fn create_not_connected_node(datastore: &DatastoreType, node: NodeData) -> Result<Vertex> {
    let v = Vertex::with_id(node.id, Identifier::new(NODE_IDENTIFIER)?);

    let q = SpecificVertexQuery::single(v.id.clone())
        .property(Identifier::new(NODE_DATA_IDENTIFIER).unwrap());

    datastore.create_vertex(&v)?;
    datastore.set_vertex_properties(q, serde_json::to_value(node.clone())?)?;

    Ok(v)
}

pub fn create_node(
    datastore: &DatastoreType,
    node_data: NodeData,
    parent_id: Uuid,
) -> Result<Node> {
    let vertex = create_not_connected_node(datastore, node_data.clone())?;

    get_node(datastore, parent_id.clone())
        .map_err(|_| anyhow::anyhow!("Parent node not found"))?;

    let t = Identifier::new(NODE_LINK_IDENTIFIER).unwrap();
    let k = EdgeKey::new(parent_id, t.clone(), node_data.id);
    datastore.create_edge(&k)?;

    let node = get_node(datastore, vertex.id)?;

    Ok(node)
}

pub fn get_node(datastore: &DatastoreType, id: Uuid) -> Result<Node> {
    let q = SpecificVertexQuery::single(id);

    let inbound_edges = datastore.get_edges(q.clone().inbound().into())?;
    let outbound_edges = datastore.get_edges(q.clone().outbound().into())?;

    let props = datastore.get_vertex_properties(
        q.clone()
            .property(Identifier::new(NODE_DATA_IDENTIFIER).unwrap()),
    )?;

    let prop = props
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

    let data: NodeData = serde_json::from_value(prop.value.clone())?;

    let node = Node {
        data,
        parents: inbound_edges
            .iter()
            .map(|e| e.key.outbound_id)
            .collect::<Vec<Uuid>>(),
        children: outbound_edges
            .iter()
            .map(|e| e.key.inbound_id)
            .collect::<Vec<Uuid>>(),
    };

    Ok(node)
}
