use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WebsiteToIndex {
    pub domain: String,
    pub added_at: DateTime<Utc>,
}
