use std::sync::Arc;

use crate::domain::entities::AlertLog;
use crate::domain::errors::DomainError;
use crate::domain::ports::AlertLogRepository;

#[derive(Clone)]
pub struct AlertsUseCase {
    alert_log_repo: Arc<dyn AlertLogRepository>,
}

impl AlertsUseCase {
    pub fn new(alert_log_repo: Arc<dyn AlertLogRepository>) -> Self {
        Self { alert_log_repo }
    }

    pub async fn list_paginated(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AlertLog>, DomainError> {
        self.alert_log_repo.list_paginated(limit, offset).await
    }
}
