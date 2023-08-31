use sqlx::Transaction;

use crate::{models::website_to_index::WebsiteToIndex, util::search_error::SearchResult};

use super::DB;

pub async fn insert_website_to_index(
    transaction: &mut Transaction<'_, DB>,
    domain: String,
) -> SearchResult<WebsiteToIndex> {
    Ok(sqlx::query_as!(
        WebsiteToIndex,
        r#"
INSERT INTO website_to_index (domain)
VALUES                       ($1)
RETURNING domain, added_at
        "#,
        domain
    )
    .fetch_one(&mut **transaction)
    .await?)
}

pub async fn get_oldest(
    transaction: &mut Transaction<'_, DB>,
) -> SearchResult<Option<WebsiteToIndex>> {
    Ok(sqlx::query_as!(
        WebsiteToIndex,
        r#"
SELECT domain, added_at
FROM website_to_index
ORDER BY added_at ASC
        "#
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
