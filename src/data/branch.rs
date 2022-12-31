use anyhow::Result;
use chrono::{DateTime, Utc};
use indradb::{Datastore, Identifier, SpecificVertexQuery, Vertex, VertexQueryExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::DatastoreType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Branch {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const BRANCH_TYPE: &str = "Branch";
const BRANCH_DATA: &str = "data";

pub fn create_not_connected_branch(datastore: &DatastoreType, branch: Branch) -> Result<Branch> {
    let v = Vertex::with_id(branch.id, Identifier::new(BRANCH_TYPE)?);

    let q =
        SpecificVertexQuery::single(v.id.clone()).property(Identifier::new(BRANCH_DATA).unwrap());

    datastore.create_vertex(&v)?;
    datastore.set_vertex_properties(q, serde_json::to_value(branch.clone())?)?;

    let saved_branch = get_branch(datastore, branch.id)?;

    Ok(saved_branch)
}

pub fn create_branch(datastore: &DatastoreType, branch: Branch) -> Result<Branch> {
    let branch = create_not_connected_branch(datastore, branch)?;

    Ok(branch)
}

pub fn get_branch(datastore: &DatastoreType, id: Uuid) -> Result<Branch> {
    let q = SpecificVertexQuery::single(id).property(Identifier::new(BRANCH_DATA).unwrap());

    let props = datastore.get_vertex_properties(q)?;

    let prop = props
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("No branch found"))?;

    let branch: Branch = serde_json::from_value(prop.value.clone())?;

    Ok(branch)
}
