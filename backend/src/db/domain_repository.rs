use sqlx::Transaction;

use crate::{models::domain::Domain, util::search_error::SearchResult};

use super::DB;

pub async fn insert_domain_to_index(
    transaction: &mut Transaction<'_, DB>,
    domain: String,
) -> SearchResult<Domain> {
    Ok(sqlx::query_as!(
        Domain,
        r#"
INSERT INTO domain (domain, indexed_at)
VALUES             ($1    , NULL)
RETURNING id, domain, indexed_at, added_at
        "#,
        domain
    )
    .fetch_one(&mut **transaction)
    .await?)
}

pub async fn set_domain_indexed(
    transaction: &mut Transaction<'_, DB>,
    domain: &Domain,
) -> SearchResult<Domain> {
    Ok(sqlx::query_as!(
        Domain,
        r#"
UPDATE domain
SET indexed_at = now()
WHERE id = $1
RETURNING id, domain, indexed_at, added_at
        "#,
        domain.id
    )
    .fetch_one(&mut **transaction)
    .await?)
}

pub async fn find_oldest_not_indexed(
    transaction: &mut Transaction<'_, DB>,
) -> SearchResult<Option<Domain>> {
    Ok(sqlx::query_as!(
        Domain,
        r#"
SELECT id, domain, indexed_at, added_at
FROM domain
WHERE indexed_at IS NULL
ORDER BY added_at ASC
        "#
    )
    .fetch_optional(&mut **transaction)
    .await?)
}

pub async fn find_by_domain(
    transaction: &mut Transaction<'_, DB>,
    domain: String,
) -> SearchResult<Option<Domain>> {
    Ok(sqlx::query_as!(
        Domain,
        r#"
SELECT id, domain, added_at, indexed_at
FROM domain
WHERE domain = $1
        "#,
        domain
    )
    .fetch_optional(&mut **transaction)
    .await?)
}

pub async fn find_non_indexed(transaction: &mut Transaction<'_, DB>) -> SearchResult<Vec<Domain>> {
    Ok(sqlx::query_as!(
        Domain,
        r#"
SELECT id, domain, added_at, indexed_at
FROM domain
WHERE indexed_at IS NULL
ORDER BY added_at ASC
        "#
    )
    .fetch_all(&mut **transaction)
    .await?)
}
