//! `repository` provides concrete implementations of the `RedirectRepository` trait
mod mongodb;
mod redis;

pub use self::mongodb::MongoRepository;
pub use self::redis::RedisRepository;
