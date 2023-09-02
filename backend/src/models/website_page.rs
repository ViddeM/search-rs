use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WebsitePage {
    pub id: Uuid,
    pub domain: Uuid,
    pub title: Option<String>,
    pub page_url: String,
    pub indexed_at: DateTime<Utc>,
}
