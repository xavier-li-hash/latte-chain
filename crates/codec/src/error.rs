#[derive(thiserror::Error, Debug)]
pub enum CodecError {
    #[error("Encode error: {0}")]
    Encode(String),

    #[error("Decode error: {0}")]
    Decode(String),
}

