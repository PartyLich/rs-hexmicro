use crate::short_url::{error, Redirect};

/// Provides a `find` method for looking up URLs based
/// on their short code and a `store` method for saving `Redirect` objects
pub trait RedirectService {
    /// Retrieves a `Redirect` based on its short code
    fn find(&self, code: &str) -> Result<Redirect, error::RedirectErr>;

    /// Save a `Redirect`
    fn store(&self, redirect: &Redirect) -> Result<Redirect, error::RedirectErr>;
}
