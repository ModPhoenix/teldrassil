use crate::{data, graphql::utils::get_datastore};
use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub parents: Vec<Branch>,
    pub children: Vec<Branch>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    parents_ids: Vec<Uuid>,
    children_ids: Vec<Uuid>,
}

#[Object]
impl Branch {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> String {
        self.name.to_string()
    }

    async fn content(&self) -> String {
        self.content.to_string()
    }

    async fn parents(&self, ctx: &Context<'_>) -> Result<Vec<Branch>> {
        let datastore = get_datastore(ctx)?;

        let parents = self
            .parents_ids
            .iter()
            .flat_map(|id| data::branch::get_branch(datastore, *id))
            .map(|branch| branch.into())
            .collect::<Vec<_>>();

        Ok(parents)
    }

    async fn children(&self, ctx: &Context<'_>) -> Result<Vec<Branch>> {
        let datastore = get_datastore(ctx)?;

        let parents = self
            .children_ids
            .iter()
            .flat_map(|id| data::branch::get_branch(datastore, *id))
            .map(|branch| branch.into())
            .collect::<Vec<_>>();

        Ok(parents)
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl From<data::branch::Branch> for Branch {
    fn from(branch: data::branch::Branch) -> Branch {
        Branch {
            id: branch.data.id,
            name: branch.data.name.clone(),
            content: branch.data.content.clone(),
            parents: vec![],
            children: vec![],
            created_at: branch.data.created_at,
            updated_at: branch.data.updated_at,
            parents_ids: branch.parents,
            children_ids: branch.children,
        }
    }
}
