use serde::{Deserialize, Serialize};

use crate::domain::node::field;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewNode {
    pub name: field::Name,
    pub content: field::Content,
    pub parent_id: Option<field::NodeId>,
}
