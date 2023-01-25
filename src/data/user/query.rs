use anyhow::Result;
use indradb::{Datastore, PropertyValueVertexQuery, SpecificVertexQuery, Vertex, VertexQueryExt};
use uuid::Uuid;

use crate::data::types::DatastoreType;

use super::{user_data_identifier, user_email_identifier, user_identifier, User};

pub fn create_user(datastore: &DatastoreType, data: User) -> Result<Vertex> {
    let v = Vertex::with_id(data.id, user_identifier());

    let q_data = SpecificVertexQuery::single(v.id.clone()).property(user_data_identifier());
    let q_email = SpecificVertexQuery::single(v.id.clone()).property(user_email_identifier());

    datastore.create_vertex(&v)?;
    datastore.set_vertex_properties(q_data, serde_json::to_value(data.clone())?)?;
    datastore.set_vertex_properties(q_email, serde_json::to_value(data.email)?)?;

    Ok(v)
}

pub fn get_user_by_id(datastore: &DatastoreType, id: Uuid) -> Result<User> {
    let q = SpecificVertexQuery::single(id);

    let props = datastore.get_all_vertex_properties(q.clone().into())?;
    let user = props
        .iter()
        .fold(None, |_: Option<User>, p| {
            let prop = p.props.iter().find(|p| p.name == user_data_identifier());

            prop.map(|p| serde_json::from_value(p.value.clone()).unwrap())
        })
        .ok_or(anyhow::anyhow!("User data not found"))?;

    Ok(user)
}

pub fn get_user_by_email(datastore: &DatastoreType, email: String) -> Result<User> {
    let q = PropertyValueVertexQuery::new(user_email_identifier(), serde_json::json!(email));

    let binding = datastore.get_all_vertex_properties(q.into())?;
    println!("{:?}", binding);
    let user = binding
        .iter()
        .fold(None, |_: Option<User>, p| {
            let prop = p.props.iter().find(|p| p.name == user_data_identifier());

            prop.map(|p| serde_json::from_value(p.value.clone()).unwrap())
        })
        .ok_or(anyhow::anyhow!("Invalid email or password"))?;

    Ok(user)
}
