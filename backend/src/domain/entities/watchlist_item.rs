use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::entities::game_price::GamePriceSnapshot;
use crate::domain::value_objects::{AppId, MoneyCents};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertDecision {
    Trigger {
        triggered_price_cents: MoneyCents,
        discount_percent: u8,
    },
    Suppress,
    ResetDeduplication,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WatchlistItem {
    pub id: String,
    pub app_id: AppId,
    pub title: String,
    pub header_image_url: String,
    pub target_price_cents: Option<MoneyCents>,
    pub currency: String,
    pub current_price_cents: Option<MoneyCents>,
    pub original_price_cents: Option<MoneyCents>,
    pub discount_percent: u8,
    pub is_free: bool,
    pub is_active: bool,
    pub last_notified_price_cents: Option<MoneyCents>,
    pub last_notified_at: Option<DateTime<Utc>>,
    pub last_checked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl WatchlistItem {
    pub fn apply_price_snapshot(&mut self, snapshot: &GamePriceSnapshot) -> AlertDecision {
        self.title = snapshot.title.clone();
        self.header_image_url = snapshot.header_image_url.clone();
        self.currency = snapshot.currency.clone();
        self.current_price_cents = Some(snapshot.current_price_cents);
        self.original_price_cents = Some(snapshot.original_price_cents);
        self.discount_percent = snapshot.discount_percent;
        self.is_free = snapshot.is_free;
        self.last_checked_at = Some(snapshot.fetched_at);

        if !self.is_active {
            return AlertDecision::Suppress;
        }

        let condition_met = match self.target_price_cents {
            Some(target) => snapshot.current_price_cents <= target,
            None => snapshot.discount_percent > 0,
        };

        if !condition_met {
            if self.last_notified_price_cents.is_some() {
                self.last_notified_price_cents = None;
                return AlertDecision::ResetDeduplication;
            }
            return AlertDecision::Suppress;
        }

        let should_notify = match self.last_notified_price_cents {
            None => true,
            Some(last_price) => snapshot.current_price_cents < last_price,
        };

        if should_notify {
            AlertDecision::Trigger {
                triggered_price_cents: snapshot.current_price_cents,
                discount_percent: snapshot.discount_percent,
            }
        } else {
            AlertDecision::Suppress
        }
    }

    pub fn mark_notified(&mut self, price: MoneyCents, at: DateTime<Utc>) {
        self.last_notified_price_cents = Some(price);
        self.last_notified_at = Some(at);
    }

    pub fn is_target_met(&self) -> bool {
        let Some(current) = self.current_price_cents else {
            return false;
        };
        match self.target_price_cents {
            Some(target) => current <= target,
            None => self.discount_percent > 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::{AlertDecision, WatchlistItem};
    use crate::domain::entities::GamePriceSnapshot;
    use crate::domain::value_objects::{AppId, MoneyCents};

    fn make_item(target_cents: Option<i64>, is_active: bool) -> WatchlistItem {
        WatchlistItem {
            id: "item-1".to_string(),
            app_id: AppId::new(1091500).expect("valid app id"),
            title: "Cyberpunk 2077".to_string(),
            header_image_url: "https://example.com/header.jpg".to_string(),
            target_price_cents: target_cents.map(|v| MoneyCents::new(v).expect("valid cents")),
            currency: "USD".to_string(),
            current_price_cents: None,
            original_price_cents: None,
            discount_percent: 0,
            is_free: false,
            is_active,
            last_notified_price_cents: None,
            last_notified_at: None,
            last_checked_at: None,
            created_at: Utc::now(),
        }
    }

    fn make_snapshot(current: i64, original: i64, discount: u8) -> GamePriceSnapshot {
        GamePriceSnapshot {
            app_id: AppId::new(1091500).expect("valid app id"),
            title: "Cyberpunk 2077".to_string(),
            header_image_url: "https://example.com/header.jpg".to_string(),
            currency: "USD".to_string(),
            current_price_cents: MoneyCents::new(current).expect("valid cents"),
            original_price_cents: MoneyCents::new(original).expect("valid cents"),
            discount_percent: discount,
            is_free: false,
            fetched_at: Utc::now(),
        }
    }

    #[test]
    fn triggers_when_price_meets_target_and_deduplicates_same_price() {
        let mut item = make_item(Some(3000), true);

        let decision = item.apply_price_snapshot(&make_snapshot(2999, 5999, 50));
        assert_eq!(
            decision,
            AlertDecision::Trigger {
                triggered_price_cents: MoneyCents::new(2999).expect("valid"),
                discount_percent: 50,
            }
        );

        item.mark_notified(MoneyCents::new(2999).expect("valid"), Utc::now());

        let repeat = item.apply_price_snapshot(&make_snapshot(2999, 5999, 50));
        assert_eq!(repeat, AlertDecision::Suppress);

        let deeper_drop = item.apply_price_snapshot(&make_snapshot(1999, 5999, 67));
        assert_eq!(
            deeper_drop,
            AlertDecision::Trigger {
                triggered_price_cents: MoneyCents::new(1999).expect("valid"),
                discount_percent: 67,
            }
        );
    }

    #[test]
    fn resets_deduplication_when_sale_ends_and_triggers_next_sale() {
        let mut item = make_item(None, true);

        let first = item.apply_price_snapshot(&make_snapshot(2999, 5999, 50));
        assert!(matches!(first, AlertDecision::Trigger { .. }));
        item.mark_notified(MoneyCents::new(2999).expect("valid"), Utc::now());

        let sale_ended = item.apply_price_snapshot(&make_snapshot(5999, 5999, 0));
        assert_eq!(sale_ended, AlertDecision::ResetDeduplication);
        assert_eq!(item.last_notified_price_cents, None);

        let next_sale = item.apply_price_snapshot(&make_snapshot(2999, 5999, 50));
        assert!(matches!(next_sale, AlertDecision::Trigger { .. }));
    }

    #[test]
    fn suppresses_alerts_when_paused() {
        let mut item = make_item(None, false);
        let decision = item.apply_price_snapshot(&make_snapshot(1999, 5999, 67));
        assert_eq!(decision, AlertDecision::Suppress);
    }
}
