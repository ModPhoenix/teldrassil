use crate::domain::node::NodeError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, NodeError> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(NodeError::EmptyContent)
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for Content {
    type Error = NodeError;

    fn try_from(content: String) -> Result<Self, Self::Error> {
        Self::new(content.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::Content;

    #[test]
    fn disallow_empty_content() {
        assert!(Content::new("").is_err());
    }
}
