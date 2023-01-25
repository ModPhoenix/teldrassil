use indradb::Identifier;

use crate::data::utils::identifier;

pub fn user_identifier() -> Identifier {
    identifier("user")
}

pub fn user_data_identifier() -> Identifier {
    identifier("user_data")
}

pub fn user_email_identifier() -> Identifier {
    identifier("user_email")
}
