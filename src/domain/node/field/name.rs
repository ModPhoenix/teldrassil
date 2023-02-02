use crate::domain::node::NodeError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Result<Self, NodeError> {
        if !name.trim().is_empty() {
            Ok(Self(name.to_owned()))
        } else {
            Err(NodeError::EmptyName)
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod test {
    use super::Name;

    #[test]
    fn disallow_empty_name() {
        assert!(Name::new("").is_err());
    }
}
