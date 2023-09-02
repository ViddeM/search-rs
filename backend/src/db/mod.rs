use sqlx::{Pool, Postgres, Transaction};

use crate::util::search_error::{SearchError, SearchResult};

pub type DB = Postgres;

pub mod domain_link_repository;
pub mod domain_repository;
pub mod website_page_repository;

pub async fn new_transaction(db_pool: &Pool<DB>) -> SearchResult<Transaction<'_, DB>> {
    match db_pool.begin().await {
        Ok(transaction) => Ok(transaction),
        Err(err) => {
            error!("Failed to create transaction: {:?}", err);
            Err(SearchError::SqlxError(err))
        }
    }
}
