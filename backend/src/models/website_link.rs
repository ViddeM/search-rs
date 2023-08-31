use sqlx::types::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WebsiteLink {
    pub parent_website: Uuid,
    pub child_website: Uuid,
}
