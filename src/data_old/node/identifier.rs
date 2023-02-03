use indradb::Identifier;

use crate::data_old::utils::identifier;

pub fn node_identifier() -> Identifier {
    identifier("node")
}

pub fn node_data_identifier() -> Identifier {
    identifier("node_data")
}

pub fn node_name_identifier() -> Identifier {
    identifier("node_name")
}

pub fn node_edge_identifier() -> Identifier {
    identifier("node_edge")
}