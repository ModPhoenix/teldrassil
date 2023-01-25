use indradb::Identifier;

use crate::data::utils::identifier;

pub fn node_identifier() -> Identifier {
    identifier("node")
}

pub fn node_data_identifier() -> Identifier {
    identifier("node_data")
}

pub fn node_edge_identifier() -> Identifier {
    identifier("node_edge")
}
