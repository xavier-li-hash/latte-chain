use thiserror::Error;
use crate::storage_error::StorageError;

#[derive(Debug, Error)]
pub enum ChainError {
    #[error("invalid parent hash")]
    InvalidParent,

    #[error("invalid block height")]
    InvalidHeight,

    #[error("block execution failed")]
    ExecutionFailed,

    #[error("state root mismatch")]
    StateRootMismatch,

    #[error("transaction root mismatch")]
    TxRootMismatch,

    #[error("block not found")]
    BlockNotFound,

    #[error("invalid block: {0}")]
    InvalidBlock(String),

    #[error("validation error")]
    ValidationError,

    #[error("block timeout error")]
    TimeoutError,

    #[error("storage error: {0}")]
    Storage(#[from] StorageError),

}
