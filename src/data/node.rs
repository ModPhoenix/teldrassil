use anyhow::Result;
use indradb::{Datastore, EdgeKey, Identifier, SpecificVertexQuery, VertexQueryExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{create_not_connected_knowledge_node, types::DatastoreType, Knowledge, User};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeData {
    Knowledge(Knowledge),
    User(User),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub data: NodeData,
    pub parents: Vec<Uuid>,
    pub children: Vec<Uuid>,
}

pub const NODE_IDENTIFIER: &str = "node";
pub const NODE_DATA_IDENTIFIER: &str = "node_data";
pub const NODE_LINK_IDENTIFIER: &str = "node_link";

pub fn create_node(datastore: &DatastoreType, data: NodeData, parent_id: Uuid) -> Result<Node> {
    // let vertex = create_not_connected_node(datastore, node_data.clone())?;

    let vertex = match data {
        NodeData::Knowledge(k) => create_not_connected_knowledge_node(datastore, k)?,
        NodeData::User(_u) => {
            // create_not_connected_user_node(datastore, u)?
            unimplemented!()
        }
    };

    get_node(datastore, parent_id.clone()).map_err(|_| anyhow::anyhow!("Parent node not found"))?;

    let t = Identifier::new(NODE_LINK_IDENTIFIER).unwrap();
    let k = EdgeKey::new(parent_id, t.clone(), vertex.id);
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

    let k: Knowledge = serde_json::from_value(prop.value.clone())?;

    let data = NodeData::Knowledge(k);

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
