#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("Sqlx Error")]
    SqlxError(#[from] sqlx::Error),
    #[error("Rocket error")]
    RocketError(#[from] rocket::Error),
}

pub type SearchResult<T> = Result<T, SearchError>;
