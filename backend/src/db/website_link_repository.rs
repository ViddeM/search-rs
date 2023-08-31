use sqlx::Transaction;

use crate::{
    models::{indexed_website::IndexedWebsite, website_link::WebsiteLink},
    util::search_error::SearchResult,
};

use super::DB;

pub async fn insert_website_link(
    transaction: &mut Transaction<'_, DB>,
    parent_website: IndexedWebsite,
    child_website: IndexedWebsite,
) -> SearchResult<WebsiteLink> {
    Ok(sqlx::query_as!(
        WebsiteLink,
        r#"
INSERT INTO website_link (parent_website, child_website)
VALUES                   ($1,             $2           )
RETURNING parent_website, child_website
        "#,
        parent_website.id,
        child_website.id,
    )
    .fetch_one(&mut **transaction)
    .await?)
}
