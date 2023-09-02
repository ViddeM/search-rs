use sqlx::types::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DomainLink {
    pub parent_domain: Uuid,
    pub child_domain: Uuid,
}
