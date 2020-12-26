/// Result type for Error
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Any kind of error encountered during parsing
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("error parsing int: {0}")]
    Int(#[from] std::num::ParseIntError),

    #[error("error parsing float: {0}")]
    Float(#[from] std::num::ParseFloatError),

    #[error("invalid hit object type: {0}")]
    InvalidObjectType(i32),

    #[error("invalid slider spline type: {0}")]
    InvalidSliderType(String),

    #[error("invalid sample set: {0}")]
    InvalidSampleSet(String),

    #[error("invalid game mode: {0}")]
    InvalidGameMode(u8),
}
