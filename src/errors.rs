/// Result type for Error
pub type ParseResult<T, E = ParseError> = std::result::Result<T, E>;

/// Any kind of error encountered during parsing
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum ParseError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("error parsing int: {0}")]
    Int(#[from] std::num::ParseIntError),

    #[error("error parsing float: {0}")]
    Float(#[from] std::num::ParseFloatError),

    #[error("missing component in color")]
    MissingColorComponent,

    #[error("invalid additions: {0}")]
    InvalidAdditions(u32),

    #[error("invalid hit object type: {0}")]
    InvalidObjectType(i32),

    #[error("invalid slider spline type: {0}")]
    InvalidSliderType(String),

    #[error("invalid sample set: {0}")]
    InvalidSampleSet(u32),

    #[error("invalid sample set: {0}")]
    InvalidSampleSetString(String),

    #[error("invalid game mode: {0}")]
    InvalidGameMode(u8),

    #[error("invalid grid size: {0}")]
    InvalidGridSize(u8),

    #[error("custom: {0}")]
    Custom(String),
}
