use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::UserError;

#[derive(Clone, Debug, Constructor, Deserialize, Serialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for UserId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl TryFrom<String> for UserId {
    type Error = UserError;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        Ok(Self(
            Uuid::parse_str(&id).map_err(|_| UserError::InvalidId(id))?,
        ))
    }
}
