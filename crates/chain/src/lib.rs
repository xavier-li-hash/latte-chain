use crate::error::ChainError;

pub mod block_executor;
pub mod blockchain;
pub mod error;
pub mod genesis;
pub mod merkle;
pub mod storage;
pub mod validator;
pub mod canonical;
pub mod storage_error;

pub type Result<T> = std::result::Result<T, ChainError>;
