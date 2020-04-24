//! Define the serializer interface our ports/adapters will need to implement
use crate::short_url::Redirect;
use std::error::Error;

/// Provides methods to serialize and deserialize `Redirect` objects
pub trait RedirectSerializer {
    /// Encode a `Redirect` to the serialization format (as a byte sequence)
    fn encode(&self, input: &Redirect) -> Result<Vec<u8>, Box<dyn Error>>;

    /// Decode the serialization format (as a byte sequence) into a `Redirect`
    fn decode(&self, input: &Vec<u8>) -> Result<Redirect, Box<dyn Error>>;
}
