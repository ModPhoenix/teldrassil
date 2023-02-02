//! Time wrapper structure.

use std::str::FromStr;

use chrono::{DateTime, Utc};
use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};

/// This type uses Utc time only.
#[derive(Clone, Debug, From, Constructor, Deserialize, Serialize)]
pub struct Time(DateTime<Utc>);

impl Time {
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }
}

impl From<Time> for DateTime<Utc> {
    fn from(time: Time) -> Self {
        time.0
    }
}

/// The format required is `YYYY-MM-DDThh:mm:ssZ`.
impl FromStr for Time {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<DateTime<Utc>>() {
            Ok(time) => Ok(time.into()),
            Err(e) => Err(e),
        }
    }
}
