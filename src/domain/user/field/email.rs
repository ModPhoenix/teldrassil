use crate::domain::user::UserError;
use serde::{Deserialize, Serialize};
use validator::validate_email;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, UserError> {
        if validate_email(email) {
            Ok(Self(email.to_owned()))
        } else {
            Err(UserError::InvalidEmail(email.to_owned()))
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for Email {
    type Error = UserError;
    fn try_from(name: String) -> Result<Self, Self::Error> {
        Self::new(name.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::Email;

    #[test]
    fn disallow_empty_name() {
        assert!(Email::new("").is_err());
    }
}
