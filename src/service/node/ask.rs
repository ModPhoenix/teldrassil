use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{domain::node::field, service::ServiceError};

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
    type Error = ServiceError;

    fn try_from(input: NewNodeInput) -> Result<Self, Self::Error> {
        let parent_id: Option<Result<Uuid, uuid::Error>> =
            input.parent_id.map(|id| Uuid::parse_str(&id));

        Ok(Self {
            name: input.name.try_into()?,
            content: field::Content::new(input.content.as_str())?,
            parent_id: parent_id.transpose()?.map(Into::into),
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
            id: Uuid::parse_str(&input.id)?.into(),
        })
    }
}

#[derive(InputObject)]
pub struct GetNodeChildrenInput {
    #[graphql(skip)]
    pub id: String,
    #[graphql(default = 0)]
    pub offset: i32,
    #[graphql(default = 20)]
    pub limit: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetNodeChildren {
    pub id: field::NodeId,
    pub offset: i32,
    pub limit: i32,
}

impl TryFrom<GetNodeChildrenInput> for GetNodeChildren {
    type Error = ServiceError;

    fn try_from(input: GetNodeChildrenInput) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&input.id)?.into(),
            offset: input.offset,
            limit: input.limit,
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
            id: Uuid::parse_str(&input.id)?.into(),
            name: input.name.map(|name| name.try_into()).transpose()?,
            content: input
                .content
                .map(|content| content.try_into())
                .transpose()?,
        })
    }
}

#[derive(InputObject)]
pub struct DeleteNodeInput {
    pub id: String,
}

pub struct DeleteNode {
    pub id: field::NodeId,
}

impl TryFrom<DeleteNodeInput> for DeleteNode {
    type Error = ServiceError;

    fn try_from(input: DeleteNodeInput) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&input.id)?.into(),
        })
    }
}
