use thiserror::Error;
#[derive(Debug, Error)]
pub enum VMError {
    #[error("out of gas")]
    OutOfGas,

    #[error("stack underflow")]
    StackUnderflow,

    #[error("invalid jump")]
    InvalidJump,

    #[error("divide by zero")]
    DivideByZero,

}