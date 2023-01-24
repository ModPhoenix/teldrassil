use anyhow::Result;
use chrono::{DateTime, Utc};
use indradb::{Datastore, Identifier, SpecificVertexQuery, Vertex, VertexQueryExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{DatastoreType, NODE_DATA_IDENTIFIER, NODE_IDENTIFIER};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Knowledge {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn create_not_connected_knowledge_node(
    datastore: &DatastoreType,
    data: Knowledge,
) -> Result<Vertex> {
    let v = Vertex::with_id(data.id, Identifier::new(NODE_IDENTIFIER)?);

    let q = SpecificVertexQuery::single(v.id.clone())
        .property(Identifier::new(NODE_DATA_IDENTIFIER).unwrap());

    datastore.create_vertex(&v)?;
    datastore.set_vertex_properties(q, serde_json::to_value(data.clone())?)?;

    Ok(v)
}
