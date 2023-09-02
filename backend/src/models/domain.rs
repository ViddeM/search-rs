use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, PartialEq, PartialOrd)]
pub struct Domain {
    pub id: Uuid,
    pub domain: String,
    pub indexed_at: Option<DateTime<Utc>>,
    pub added_at: DateTime<Utc>,
}
