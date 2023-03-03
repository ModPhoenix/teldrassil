use crate::domain::user::UserError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Password(String);

impl Password {
    pub fn new(name: &str) -> Result<Self, UserError> {
        if !name.trim().is_empty() {
            Ok(Self(name.to_owned()))
        } else {
            Err(UserError::InvalidUsername(name.to_owned()))
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for Password {
    type Error = UserError;
    fn try_from(name: String) -> Result<Self, Self::Error> {
        Self::new(name.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::Password;

    #[test]
    fn disallow_empty_name() {
        assert!(Password::new("").is_err());
    }
}
