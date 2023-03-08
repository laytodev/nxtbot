use thiserror::Error;

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("internal I/O error")]
    IoError(#[from] std::io::Error),

    #[error("unable to resolve symbol '{}'", 0)]
    NoSymbolFound(&'static str),
}