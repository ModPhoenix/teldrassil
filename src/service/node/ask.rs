use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

use crate::{
    data::DbId,
    domain::{self, node::field},
    service::ServiceError,
};

#[derive(InputObject)]
pub struct NewNodeInput {
    pub name: String,
    pub content: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewNode {
    pub name: field::Name,
    pub content: field::Content,
    pub parent_id: Option<field::NodeId>,
}

impl TryFrom<NewNodeInput> for NewNode {
    type Error = domain::NodeError;

    fn try_from(input: NewNodeInput) -> Result<Self, Self::Error> {
        Ok(Self {
            name: input.name.try_into()?,
            content: field::Content::new(input.content.as_str())?,
            parent_id: input.parent_id.map(|id| field::NodeId::new(id.into())),
        })
    }
}

#[derive(InputObject)]
pub struct GetNodeInput {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetNode {
    pub id: field::NodeId,
}

impl TryFrom<GetNodeInput> for GetNode {
    type Error = ServiceError;

    fn try_from(input: GetNodeInput) -> Result<Self, Self::Error> {
        Ok(Self {
            id: DbId::from(input.id).into(),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetNodeMeanings {
    pub id: field::NodeId,
    pub name: field::Name,
}

#[derive(InputObject)]
pub struct UpdateNodeInput {
    pub id: String,
    pub name: Option<String>,
    pub content: Option<String>,
}

pub struct UpdateNode {
    pub id: field::NodeId,
    pub name: Option<field::Name>,
    pub content: Option<field::Content>,
}

impl TryFrom<UpdateNodeInput> for UpdateNode {
    type Error = ServiceError;

    fn try_from(input: UpdateNodeInput) -> Result<Self, Self::Error> {
        Ok(Self {
            id: DbId::from(input.id).into(),
            name: input.name.map(|name| name.try_into()).transpose()?,
            content: input
                .content
                .map(|content| content.try_into())
                .transpose()?,
        })
    }
}
