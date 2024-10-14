use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Request Error when talking to qbittorrent: {0}")]
    ReqErr(#[from] reqwest::Error),
    #[error("Could not convert reqwest header to string: {0}")]
    ToStringError(#[from] reqwest::header::ToStrError),
    #[error("Serde json could not correctly deserialize: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Header value was malformed: {0}")]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Header value was not correctly set - are your username and password correct?")]
    MissingHeaders,
    #[error("Cookie value was not correctly set")]
    MissingCookie,
    #[error("SLICE ERROR ??")]
    SliceError,
    #[error("Bad response from server")]
    BadResponse,
}

pub type Result<T> = std::result::Result<T, Error>;
