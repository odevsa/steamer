use async_trait::async_trait;

use crate::domain::entities::{AlertLog, NewAlertLog};
use crate::domain::errors::DomainError;

#[async_trait]
pub trait AlertLogRepository: Send + Sync {
    async fn insert(&self, entry: &NewAlertLog) -> Result<(), DomainError>;
    async fn list_paginated(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AlertLog>, DomainError>;
}
