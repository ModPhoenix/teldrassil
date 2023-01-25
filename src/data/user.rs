use anyhow::Result;
use chrono::{DateTime, Utc};
use indradb::{
    Datastore, Identifier, PropertyValueVertexQuery, SpecificVertexQuery, Vertex, VertexQueryExt,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{DatastoreType, NODE_IDENTIFIER};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, username: String, password: String) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            email,
            username,
            password,
            created_at: now,
            updated_at: now,
        }
    }
}

pub const USER_DATA_IDENTIFIER: &str = "user_data";
pub const USER_EMAIL_IDENTIFIER: &str = "user_email";

pub fn get_user_data_identifier() -> Identifier {
    Identifier::new(USER_DATA_IDENTIFIER).unwrap()
}

pub fn get_user_email_identifier() -> Identifier {
    Identifier::new(USER_EMAIL_IDENTIFIER).unwrap()
}

pub fn create_not_connected_user_node(datastore: &DatastoreType, data: User) -> Result<Vertex> {
    let v = Vertex::with_id(data.id, Identifier::new(NODE_IDENTIFIER)?);

    let q_data = SpecificVertexQuery::single(v.id.clone()).property(get_user_data_identifier());
    let q_email = SpecificVertexQuery::single(v.id.clone()).property(get_user_email_identifier());

    datastore.create_vertex(&v)?;
    datastore.set_vertex_properties(q_data, serde_json::to_value(data.clone())?)?;
    datastore.set_vertex_properties(q_email, serde_json::to_value(data.email)?)?;

    Ok(v)
}

pub fn get_user_node_by_email(datastore: &DatastoreType, email: String) -> Result<User> {
    let q = PropertyValueVertexQuery::new(get_user_email_identifier(), serde_json::json!(email));

    let binding = datastore.get_all_vertex_properties(q.into())?;
    let user = binding
        .iter()
        .fold(None, |_: Option<User>, p| {
            let prop = p
                .props
                .iter()
                .find(|p| p.name == get_user_data_identifier());

            prop.map(|p| serde_json::from_value(p.value.clone()).unwrap())
        })
        .ok_or(anyhow::anyhow!("Invalid email or password"))?;

    Ok(user)
}
