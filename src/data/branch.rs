use anyhow::Result;
use chrono::{DateTime, Utc};
use indradb::{Datastore, EdgeKey, Identifier, SpecificVertexQuery, Vertex, VertexQueryExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::DatastoreType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BranchData {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Branch {
    pub data: BranchData,
    pub parents: Vec<Uuid>,
    pub children: Vec<Uuid>,
}

const BRANCH_TYPE: &str = "Branch";
const BRANCH_DATA: &str = "data";
const BRANCH_EDGE: &str = "Branch_edge";

pub fn create_not_connected_branch(
    datastore: &DatastoreType,
    branch: BranchData,
) -> Result<Vertex> {
    let v = Vertex::with_id(branch.id, Identifier::new(BRANCH_TYPE)?);

    let q =
        SpecificVertexQuery::single(v.id.clone()).property(Identifier::new(BRANCH_DATA).unwrap());

    datastore.create_vertex(&v)?;
    datastore.set_vertex_properties(q, serde_json::to_value(branch.clone())?)?;

    Ok(v)
}

pub fn create_branch(
    datastore: &DatastoreType,
    branch_data: BranchData,
    parent_id: Uuid,
) -> Result<Branch> {
    let vertex = create_not_connected_branch(datastore, branch_data.clone())?;

    get_branch(datastore, parent_id.clone())
        .map_err(|_| anyhow::anyhow!("Parent branch not found"))?;

    let t = Identifier::new(BRANCH_EDGE).unwrap();
    let k = EdgeKey::new(parent_id, t.clone(), branch_data.id);
    datastore.create_edge(&k)?;

    let branch = get_branch(datastore, vertex.id)?;

    Ok(branch)
}

pub fn get_branch(datastore: &DatastoreType, id: Uuid) -> Result<Branch> {
    let q = SpecificVertexQuery::single(id);

    let inbound_edges = datastore.get_edges(q.clone().inbound().into())?;
    let outbound_edges = datastore.get_edges(q.clone().outbound().into())?;

    let props = datastore
        .get_vertex_properties(q.clone().property(Identifier::new(BRANCH_DATA).unwrap()))?;

    let prop = props
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("No branch found"))?;

    let data: BranchData = serde_json::from_value(prop.value.clone())?;

    let branch = Branch {
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

    Ok(branch)
}
