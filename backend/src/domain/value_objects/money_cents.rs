use serde::{Deserialize, Serialize};

use crate::domain::errors::DomainError;

const MAX_REASONABLE_CENTS: i64 = 100_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MoneyCents(i64);

impl MoneyCents {
    pub fn new(value: i64) -> Result<Self, DomainError> {
        if !(0..=MAX_REASONABLE_CENTS).contains(&value) {
            return Err(DomainError::Validation(
                "price cents must be between 0 and 100000000".to_string(),
            ));
        }
        Ok(Self(value))
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for MoneyCents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
