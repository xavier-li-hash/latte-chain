use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BlockchainError {
    InvalidTransaction,
    InsufficientGas,
    InvalidReceipt,
    InvalidSignature,
    UnknownError(String),
}

impl std::error::Error for BlockchainError {}

impl fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BlockchainError::InvalidTransaction => {
                write!(f, "Invalid transaction")
            }
            BlockchainError::InsufficientGas => {
                write!(f, "Insufficient gas")
            }
            BlockchainError::InvalidReceipt => {
                write!(f, "Invalid receipt")
            }
            BlockchainError::InvalidSignature => {
                write!(f, "Invalid signature")
            }
            BlockchainError::UnknownError(msg) => {
                write!(f, "Unknown error: {}", msg)
            }
        }
    }
}
