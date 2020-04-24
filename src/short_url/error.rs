//! Define custom `Error` types for the short url domain
use std::{error::Error, fmt};

/// An enumeration of the possible error types the url shortener may produce.
#[derive(Debug, Clone, PartialEq)]
pub enum RedirectErr {
    /// `Redirect` with the specified short code could not be located
    NotFound,

    /// Invalid data/request/etc according to context
    Invalid,

    /// Generic catch all error
    ServerErr,
}

impl fmt::Display for RedirectErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RedirectErr::NotFound => write!(f, "Redirect not found"),
            RedirectErr::Invalid => write!(f, "Redirect Invalid"),
            RedirectErr::ServerErr => write!(f, "A server error occured"),
        }
    }
}

impl Error for RedirectErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
