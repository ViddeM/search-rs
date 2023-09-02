use sqlx::Transaction;

use crate::{
    models::{domain::Domain, website_page::WebsitePage},
    util::search_error::SearchResult,
};

use super::DB;

pub async fn insert_website_page(
    transaction: &mut Transaction<'_, DB>,
    domain: &Domain,
    title: Option<String>,
    url: String,
) -> SearchResult<WebsitePage> {
    Ok(sqlx::query_as!(
        WebsitePage,
        r#" 
INSERT INTO website_page (domain, title, page_url)
VALUES                   ($1,     $2,    $3      )
RETURNING id, domain, title, page_url, indexed_at
        "#,
        domain.id,
        title,
        url
    )
    .fetch_one(&mut **transaction)
    .await?)
}

pub async fn find_website_page_by_url(
    transaction: &mut Transaction<'_, DB>,
    url: String,
) -> SearchResult<Option<WebsitePage>> {
    Ok(sqlx::query_as!(
        WebsitePage,
        r#"
SELECT id, domain, title, page_url, indexed_at
FROM website_page
WHERE page_url = $1
        "#,
        url
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
