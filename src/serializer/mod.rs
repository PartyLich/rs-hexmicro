//! `serializer` provides concrete implementations of the `RedirectSerializer` interface
mod json;

pub use self::json::JsonSerializer;
