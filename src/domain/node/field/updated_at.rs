use crate::domain::time::Time;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct UpdatedAt(Time);

impl UpdatedAt {
    pub fn into_inner(self) -> Time {
        self.0
    }
}
