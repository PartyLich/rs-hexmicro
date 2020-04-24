//! `repository` provides concrete implementations of the `RedirectRepository` trait
mod redis;

pub use self::redis::RedisRepository;
