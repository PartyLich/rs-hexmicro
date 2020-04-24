//! Repository
//!
//! Define the repository interface our ports/adapters will need to implement
use crate::short_url::{error, Redirect};

/// Provides a `find` method for looking up URLs based
/// on their short code and a `store` method for saving `Redirect` objects
pub trait RedirectRepository {
    /// Retrieve a URL from the repo based on its short code
    fn find(&self, code: &str) -> Result<Redirect, error::RedirectErr>;

    /// Save a `Redirect` object to the repository
    fn store(&self, redirect: &Redirect) -> Result<(), error::RedirectErr>;
}
