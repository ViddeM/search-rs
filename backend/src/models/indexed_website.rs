use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct IndexedWebsite {
    pub id: Uuid,
    pub title: Option<String>,
    pub domain: String,
    pub indexed_at: DateTime<Utc>,
}
