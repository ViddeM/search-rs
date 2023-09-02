use sqlx::Transaction;

use crate::{
    models::{domain::Domain, domain_link::DomainLink},
    util::search_error::SearchResult,
};

use super::DB;

pub async fn insert_domain_link(
    transaction: &mut Transaction<'_, DB>,
    parent_domain: &Domain,
    child_domain: &Domain,
) -> SearchResult<DomainLink> {
    Ok(sqlx::query_as!(
        DomainLink,
        r#"
INSERT INTO domain_link (parent_domain, child_domain)
VALUES                  ($1,             $2           )
RETURNING parent_domain, child_domain
        "#,
        parent_domain.id,
        child_domain.id,
    )
    .fetch_one(&mut **transaction)
    .await?)
}
