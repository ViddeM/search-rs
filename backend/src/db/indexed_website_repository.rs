use sqlx::Transaction;

use crate::{models::indexed_website::IndexedWebsite, util::search_error::SearchResult};

use super::DB;

pub async fn insert_indexed_website(
    transaction: &mut Transaction<'_, DB>,
    domain: String,
    title: Option<String>,
) -> SearchResult<IndexedWebsite> {
    Ok(sqlx::query_as!(
        IndexedWebsite,
        r#" 
INSERT INTO indexed_website (domain, title)
VALUES                      ($1,    $2    )
RETURNING id, title, domain, indexed_at
        "#,
        domain,
        title,
    )
    .fetch_one(&mut **transaction)
    .await?)
}
