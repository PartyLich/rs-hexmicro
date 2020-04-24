use std::{collections::HashMap, error::Error};

use redis::Commands;

use crate::short_url::{error::RedirectErr, Redirect, RedirectRepository};

impl From<redis::RedisError> for RedirectErr {
    fn from(_: redis::RedisError) -> RedirectErr {
        RedirectErr::ServerErr
    }
}

impl From<std::num::ParseIntError> for RedirectErr {
    fn from(_: std::num::ParseIntError) -> RedirectErr {
        RedirectErr::ServerErr
    }
}

/// A Redis backed `RedirectRepository` object
#[derive(Debug)]
pub struct RedisRepository {
    client: redis::Client,
}

impl RedisRepository {
    /// Initialize a new Redis client
    fn new_redis_client(redis_url: &str) -> Result<redis::Client, Box<dyn Error>> {
        let client = redis::Client::open(redis_url)?;

        // create client and check connection
        client.get_connection()?;

        Ok(client)
    }

    /// Create an instance of RedirectRepository backed by Redis
    pub fn new(redis_url: &str) -> Result<Box<RedisRepository>, Box<dyn Error>> {
        let client = RedisRepository::new_redis_client(redis_url)?;

        Ok(Box::new(RedisRepository { client: client }))
    }

    /// Generate key for Redis store
    fn generate_key(code: &str) -> String {
        format!("redirect:{}", code)
    }

    /// Parse HashMap returned by Redis into a Redirect object or die tryin'
    fn parse_hashmap(data: &HashMap<String, String>) -> Result<Redirect, RedirectErr> {
        let created_at = data
            .get(&String::from("createdAt"))
            .ok_or(RedirectErr::NotFound)?
            .parse::<i64>()?;
        let url = data
            .get(&String::from("url"))
            .ok_or(RedirectErr::NotFound)?
            .to_string();
        let code = data
            .get(&String::from("code"))
            .ok_or(RedirectErr::NotFound)?
            .to_string();

        Ok(Redirect {
            code,
            created_at,
            url,
        })
    }
}

impl RedirectRepository for RedisRepository {
    /// Look up URL based on its short code
    fn find(&self, code: &str) -> Result<Redirect, RedirectErr> {
        let key = RedisRepository::generate_key(code);

        // search database for code
        let mut conn = self.client.get_connection()?;
        let data: HashMap<String, String> = conn.hgetall(key)?;
        if data.len() == 0 {
            // redirect not found
            eprintln!("Unable to find redirect for code: {}", code);
            return Err(RedirectErr::NotFound);
        }

        RedisRepository::parse_hashmap(&data)
    }

    /// Save Redirect object
    fn store(&self, redirect: &Redirect) -> Result<(), RedirectErr> {
        let key = RedisRepository::generate_key(&redirect.code);
        let mut conn = self.client.get_connection()?;

        let data = vec![
            ("code", redirect.code.clone()),
            ("url", redirect.url.clone()),
            ("createdAt", redirect.created_at.to_string()),
        ];

        // save redirect to the redis store
        conn.hset_multiple(key, &data)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_url() {
        let expected = "Client { connection_info: ConnectionInfo { addr: Tcp(\"localhost\", 6379), db: 0, passwd: None } }";
        let args = "redis://localhost:6379";
        let actual = RedisRepository::new_redis_client(args).unwrap();

        assert_eq!(expected, format!("{:?}", actual));
    }

    #[test]
    fn good_config() {
        let expected = "RedisRepository { client: Client { connection_info: ConnectionInfo { addr: Tcp(\"localhost\", 6379), db: 0, passwd: None } } }";
        let args = "redis://localhost:6379";
        let actual = RedisRepository::new(args).unwrap();

        assert_eq!(expected, format!("{:?}", actual));
    }

    #[test]
    fn bad_url() {
        let expected = "Redis URL did not parse";
        let args = "";
        let actual = RedisRepository::new(args).unwrap_err();

        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn make_key() {
        let expected = "redirect:foo";
        let actual = RedisRepository::generate_key("foo");

        assert_eq!(actual, expected);
    }
}
