//! `serializer` provides concrete implementations of the `RedirectSerializer` interface
mod json;
mod msgpack;

pub use self::json::JsonSerializer;
pub use msgpack::MsgpackSerializer;
