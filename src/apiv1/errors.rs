/// An error that could occur when using the API.
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("hyper error: {0}")]
    Hyper(#[from] hyper::Error),

    #[error("invalid uri: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),

    #[error("serde_json error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type that defaults to the API error.
pub type Result<T, E = Error> = std::result::Result<T, E>;
