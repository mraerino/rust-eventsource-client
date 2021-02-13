/// Error type returned from this library's functions.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    HttpRequest(Box<dyn std::error::Error + Send + 'static>),
    #[error("error reading from the HTTP response body: {0}")]
    HttpStream(Box<dyn std::error::Error + Send + 'static>),
    /// e.g. in the middle of an event.
    #[error("HTTP response stream ended unexpectedly")]
    UnexpectedEof,
    /// Encountered a line not conforming to the SSE protocol.
    #[error("Invalid line encountered")]
    InvalidLine(String),
    /// Encountered an event type that is not a valid UTF-8 byte sequence.
    #[error("Event type is not valid UTF-8")]
    InvalidEventType(std::str::Utf8Error),
    /// An unexpected failure occurred.
    #[error("Unexpected failure: {0}")]
    Unexpected(#[from] Box<dyn std::error::Error + Send + 'static>),
}

impl PartialEq<Error> for Error {
    fn eq(&self, other: &Error) -> bool {
        use Error::*;
        if let (InvalidLine(msg1), InvalidLine(msg2)) = (self, other) {
            return msg1 == msg2;
        } else if let (UnexpectedEof, UnexpectedEof) = (self, other) {
            return true;
        }
        false
    }
}

impl Error {
    pub fn is_http_stream_error(&self) -> bool {
        if let Error::HttpStream(_) = self {
            return true;
        }
        false
    }

    pub fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::HttpRequest(err) => Some(err.as_ref()),
            Error::HttpStream(err) => Some(err.as_ref()),
            Error::Unexpected(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
