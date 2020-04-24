//! `short_url` defines the domain logic and ports for a url shortening service.

use std::time::SystemTime;

use shortid;

pub mod error;
mod model;
mod repository;
mod serializer;
mod service;

pub use model::Redirect;
pub use repository::RedirectRepository;
pub use serializer::RedirectSerializer;
pub use service::RedirectService;

/// A URL redirect service
pub struct Service {
    redirect_repo: Box<dyn RedirectRepository + Send>,
}

impl Service {
    /// Creates an instance of the `RedirectService` interface
    pub fn new(redirect_repo: Box<dyn RedirectRepository + Send>) -> Self {
        Service {
            redirect_repo: redirect_repo,
        }
    }
}

impl RedirectService for Service {
    /// Retrieves URLs based on their short code
    fn find(&self, code: &str) -> Result<Redirect, error::RedirectErr> {
        self.redirect_repo.find(code)
    }

    /// Saves `Redirect` objects
    fn store(&self, redirect: &Redirect) -> Result<Redirect, error::RedirectErr> {
        let created_at = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Broken clocks")
            .as_secs();
        let code =
            to_string(&shortid::next_short_64(created_at).expect("Shortid generation failure"));

        let new_redirect = Redirect {
            created_at: created_at as i64,
            code,
            url: redirect.url.clone(),
        };
        self.redirect_repo.store(&new_redirect)?;

        Ok(new_redirect)
    }
}

/// Convert an slice of `u8` to `String`
fn to_string(src: &[u8]) -> String {
    src.into_iter().map(|val| format!("{:0>2x}", val)).collect()
}
