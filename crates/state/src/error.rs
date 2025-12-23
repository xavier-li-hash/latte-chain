use thiserror::Error;
#[derive(Debug, Error)]
pub enum StateError {
    #[error("invalid state nonce")]
    InvalidNonce,
    #[error("insufficient balance")]
    InsufficientBalance,
    #[error("vm execution failed")]
    VmExecutionFailed,
}