use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("database error: {0}")]
    Db(String),

    #[error("block query failed: {0}")]
    BlockGetFailed(String),

    #[error("block not found")]
    BlockNotFound,

    #[error("block save failed: {0}")]
    BlockSaveFailed(String),

    #[error("corrupted data")]
    CorruptedData,
}
