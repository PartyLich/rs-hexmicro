use serde::{Deserialize, Serialize};

/// A URL redirect model
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Redirect {
    /// Creation timestamp
    ///
    /// An offset from the Unix epoch representing creation of this `Redirect`
    #[serde(default)]
    pub created_at: i64,

    /// Lookup key
    ///
    /// This is the unique(?) identifier used to access a given redirect. The "short" part of a shortened Url
    #[serde(default)]
    pub code: String,

    /// Destination Url
    pub url: String,
}
