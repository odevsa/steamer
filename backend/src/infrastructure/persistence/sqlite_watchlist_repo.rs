use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool};

use crate::domain::entities::{PriceHistoryPoint, WatchlistItem};
use crate::domain::errors::DomainError;
use crate::domain::ports::{PriceHistoryRepository, WatchlistRepository};
use crate::domain::value_objects::{AppId, MoneyCents};

#[derive(FromRow)]
struct WatchlistRow {
    id: String,
    app_id: i64,
    title: String,
    header_image_url: String,
    target_price_cents: Option<i64>,
    currency: String,
    current_price_cents: Option<i64>,
    original_price_cents: Option<i64>,
    discount_percent: i64,
    is_free: i64,
    is_active: i64,
    last_notified_price_cents: Option<i64>,
    last_notified_at: Option<String>,
    last_checked_at: Option<String>,
    created_at: String,
}

impl TryFrom<WatchlistRow> for WatchlistItem {
    type Error = DomainError;

    fn try_from(row: WatchlistRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            app_id: AppId::new(row.app_id)?,
            title: row.title,
            header_image_url: row.header_image_url,
            target_price_cents: row.target_price_cents.map(MoneyCents::new).transpose()?,
            currency: row.currency,
            current_price_cents: row.current_price_cents.map(MoneyCents::new).transpose()?,
            original_price_cents: row.original_price_cents.map(MoneyCents::new).transpose()?,
            discount_percent: u8::try_from(row.discount_percent).unwrap_or(0).min(100),
            is_free: row.is_free != 0,
            is_active: row.is_active != 0,
            last_notified_price_cents: row
                .last_notified_price_cents
                .map(MoneyCents::new)
                .transpose()?,
            last_notified_at: row
                .last_notified_at
                .as_deref()
                .map(parse_rfc3339)
                .transpose()?,
            last_checked_at: row
                .last_checked_at
                .as_deref()
                .map(parse_rfc3339)
                .transpose()?,
            created_at: parse_rfc3339(&row.created_at)?,
        })
    }
}

#[derive(FromRow)]
struct PriceHistoryRow {
    id: i64,
    watchlist_item_id: String,
    price_cents: i64,
    original_price_cents: i64,
    discount_percent: i64,
    recorded_at: String,
}

impl TryFrom<PriceHistoryRow> for PriceHistoryPoint {
    type Error = DomainError;

    fn try_from(row: PriceHistoryRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            watchlist_item_id: row.watchlist_item_id,
            price_cents: MoneyCents::new(row.price_cents)?,
            original_price_cents: MoneyCents::new(row.original_price_cents)?,
            discount_percent: u8::try_from(row.discount_percent).unwrap_or(0).min(100),
            recorded_at: parse_rfc3339(&row.recorded_at)?,
        })
    }
}

pub struct SqliteWatchlistRepository {
    pool: SqlitePool,
}

impl SqliteWatchlistRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

const SELECT_WATCHLIST_COLUMNS: &str = "id, app_id, title, header_image_url, target_price_cents, currency, current_price_cents, original_price_cents, discount_percent, is_free, is_active, last_notified_price_cents, last_notified_at, last_checked_at, created_at";

#[async_trait]
impl WatchlistRepository for SqliteWatchlistRepository {
    async fn list_all(&self) -> Result<Vec<WatchlistItem>, DomainError> {
        let query = format!(
            "SELECT {SELECT_WATCHLIST_COLUMNS} FROM watchlist_items ORDER BY created_at DESC"
        );
        let rows: Vec<WatchlistRow> = sqlx::query_as(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::Storage(format!("failed to query watchlist: {e}")))?;
        rows.into_iter().map(WatchlistItem::try_from).collect()
    }

    async fn list_active(&self) -> Result<Vec<WatchlistItem>, DomainError> {
        let query = format!(
            "SELECT {SELECT_WATCHLIST_COLUMNS} FROM watchlist_items WHERE is_active = 1 ORDER BY created_at ASC"
        );
        let rows: Vec<WatchlistRow> = sqlx::query_as(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::Storage(format!("failed to query active watchlist: {e}")))?;
        rows.into_iter().map(WatchlistItem::try_from).collect()
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<WatchlistItem>, DomainError> {
        let query = format!("SELECT {SELECT_WATCHLIST_COLUMNS} FROM watchlist_items WHERE id = ?1");
        let row: Option<WatchlistRow> = sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Storage(format!("failed to find watchlist item: {e}")))?;
        row.map(WatchlistItem::try_from).transpose()
    }

    async fn find_by_app_id(&self, app_id: AppId) -> Result<Option<WatchlistItem>, DomainError> {
        let query =
            format!("SELECT {SELECT_WATCHLIST_COLUMNS} FROM watchlist_items WHERE app_id = ?1");
        let row: Option<WatchlistRow> = sqlx::query_as(&query)
            .bind(app_id.value())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Storage(format!("failed to find watchlist app_id: {e}")))?;
        row.map(WatchlistItem::try_from).transpose()
    }

    async fn insert(&self, item: &WatchlistItem) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO watchlist_items (id, app_id, title, header_image_url, target_price_cents, currency, current_price_cents, original_price_cents, discount_percent, is_free, is_active, last_notified_price_cents, last_notified_at, last_checked_at, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        )
        .bind(&item.id)
        .bind(item.app_id.value())
        .bind(&item.title)
        .bind(&item.header_image_url)
        .bind(item.target_price_cents.map(MoneyCents::value))
        .bind(&item.currency)
        .bind(item.current_price_cents.map(MoneyCents::value))
        .bind(item.original_price_cents.map(MoneyCents::value))
        .bind(i64::from(item.discount_percent))
        .bind(i64::from(item.is_free))
        .bind(i64::from(item.is_active))
        .bind(item.last_notified_price_cents.map(MoneyCents::value))
        .bind(item.last_notified_at.map(|d| d.to_rfc3339()))
        .bind(item.last_checked_at.map(|d| d.to_rfc3339()))
        .bind(item.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(map_insert_error)?;
        Ok(())
    }

    async fn update(&self, item: &WatchlistItem) -> Result<(), DomainError> {
        let result = sqlx::query(
            "UPDATE watchlist_items SET title = ?1, header_image_url = ?2, target_price_cents = ?3, currency = ?4, current_price_cents = ?5, original_price_cents = ?6, discount_percent = ?7, is_free = ?8, is_active = ?9, last_notified_price_cents = ?10, last_notified_at = ?11, last_checked_at = ?12 WHERE id = ?13",
        )
        .bind(&item.title)
        .bind(&item.header_image_url)
        .bind(item.target_price_cents.map(MoneyCents::value))
        .bind(&item.currency)
        .bind(item.current_price_cents.map(MoneyCents::value))
        .bind(item.original_price_cents.map(MoneyCents::value))
        .bind(i64::from(item.discount_percent))
        .bind(i64::from(item.is_free))
        .bind(i64::from(item.is_active))
        .bind(item.last_notified_price_cents.map(MoneyCents::value))
        .bind(item.last_notified_at.map(|d| d.to_rfc3339()))
        .bind(item.last_checked_at.map(|d| d.to_rfc3339()))
        .bind(&item.id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("failed to update watchlist item: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(
                "watchlist item not found".to_string(),
            ));
        }
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<bool, DomainError> {
        let result = sqlx::query("DELETE FROM watchlist_items WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Storage(format!("failed to delete watchlist item: {e}")))?;
        Ok(result.rows_affected() > 0)
    }
}

#[async_trait]
impl PriceHistoryRepository for SqliteWatchlistRepository {
    async fn record(
        &self,
        watchlist_item_id: &str,
        price_cents: MoneyCents,
        original_price_cents: MoneyCents,
        discount_percent: u8,
    ) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO price_history (watchlist_item_id, price_cents, original_price_cents, discount_percent, recorded_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        )
        .bind(watchlist_item_id)
        .bind(price_cents.value())
        .bind(original_price_cents.value())
        .bind(i64::from(discount_percent))
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("failed to record price history: {e}")))?;
        Ok(())
    }

    async fn list_for_item(
        &self,
        watchlist_item_id: &str,
        limit: usize,
    ) -> Result<Vec<PriceHistoryPoint>, DomainError> {
        let bounded = i64::try_from(limit.clamp(1, 500)).unwrap_or(100);
        let rows: Vec<PriceHistoryRow> = sqlx::query_as(
            "SELECT id, watchlist_item_id, price_cents, original_price_cents, discount_percent, recorded_at FROM price_history WHERE watchlist_item_id = ?1 ORDER BY recorded_at DESC LIMIT ?2",
        )
        .bind(watchlist_item_id)
        .bind(bounded)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Storage(format!("failed to query price history: {e}")))?;
        rows.into_iter().map(PriceHistoryPoint::try_from).collect()
    }
}

fn map_insert_error(err: sqlx::Error) -> DomainError {
    if let sqlx::Error::Database(db_err) = &err {
        if db_err.is_unique_violation() {
            return DomainError::Conflict("game is already in watchlist".to_string());
        }
    }
    DomainError::Storage(format!("failed to insert watchlist item: {err}"))
}

fn parse_rfc3339(raw: &str) -> Result<DateTime<Utc>, DomainError> {
    DateTime::parse_from_rfc3339(raw)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| DomainError::Storage("invalid timestamp stored in database".to_string()))
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::SqliteWatchlistRepository;
    use crate::domain::entities::WatchlistItem;
    use crate::domain::ports::{PriceHistoryRepository, WatchlistRepository};
    use crate::domain::value_objects::{AppId, MoneyCents};
    use crate::infrastructure::persistence::sqlite_pool::init_in_memory_pool;

    #[tokio::test]
    async fn crud_and_price_history_round_trip() {
        let pool = init_in_memory_pool().await;
        let repo = SqliteWatchlistRepository::new(pool);

        let item = WatchlistItem {
            id: "w-1".to_string(),
            app_id: AppId::new(1091500).expect("valid"),
            title: "Cyberpunk 2077".to_string(),
            header_image_url: "https://example.com/header.jpg".to_string(),
            target_price_cents: Some(MoneyCents::new(2500).expect("valid")),
            currency: "USD".to_string(),
            current_price_cents: Some(MoneyCents::new(5999).expect("valid")),
            original_price_cents: Some(MoneyCents::new(5999).expect("valid")),
            discount_percent: 0,
            is_free: false,
            is_active: true,
            last_notified_price_cents: None,
            last_notified_at: None,
            last_checked_at: Some(Utc::now()),
            created_at: Utc::now(),
        };

        repo.insert(&item).await.expect("insert");
        let fetched = repo.find_by_id("w-1").await.expect("find").expect("exists");
        assert_eq!(fetched.title, "Cyberpunk 2077");

        repo.record(
            "w-1",
            MoneyCents::new(2999).expect("valid"),
            MoneyCents::new(5999).expect("valid"),
            50,
        )
        .await
        .expect("record history");
        let history = repo.list_for_item("w-1", 10).await.expect("list history");
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].discount_percent, 50);

        assert!(repo.delete("w-1").await.expect("delete"));
        assert!(repo.find_by_id("w-1").await.expect("find").is_none());
    }
}
