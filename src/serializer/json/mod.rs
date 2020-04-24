//!  json
//!
//!  `json` provides a concrete implementation of the RedirectSerializer interface
use serde_json;
use std::error::Error;

use crate::short_url::{Redirect, RedirectSerializer};

/// A concrete implementation of the `RedirectSerializer` interface that uses json as its protocol
pub struct JsonSerializer();

impl RedirectSerializer for JsonSerializer {
    /// Converts a `Redirect` to a JSON byte sequence
    fn encode(&self, input: &Redirect) -> Result<Vec<u8>, Box<dyn Error>> {
        serde_json::to_vec(input).map_err(|e| e.into())
    }

    /// Converts a JSON byte sequence to a `Redirect`
    fn decode(&self, input: &Vec<u8>) -> Result<Redirect, Box<dyn Error>> {
        serde_json::from_slice(input).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        let serializer = JsonSerializer {};
        let expected = Redirect {
            code: String::from("foo"),
            created_at: 0,
            url: String::from("bar"),
        };
        let encoded = serializer.encode(&expected).unwrap();
        let actual = serializer.decode(&encoded).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn encodes() {
        let serializer = JsonSerializer {};
        let expected = "[123, 34, 99, 114, 101, 97, 116, 101, 100, 95, 97, 116, 34, 58, 48, 44, 34, 99, 111, 100, 101, 34, 58, 34, 102, 111, 111, 34, 44, 34, 117, 114, 108, 34, 58, 34, 98, 97, 114, 34, 125]";
        let data = Redirect {
            code: String::from("foo"),
            created_at: 0,
            url: String::from("bar"),
        };
        let actual = serializer.encode(&data).unwrap();
        assert_eq!(expected, format!("{:?}", actual));
    }

    #[test]
    fn decodes() {
        let serializer = JsonSerializer {};
        let data: Vec<u8> = vec![
            123, 34, 99, 114, 101, 97, 116, 101, 100, 95, 97, 116, 34, 58, 48, 44, 34, 99, 111,
            100, 101, 34, 58, 34, 102, 111, 111, 34, 44, 34, 117, 114, 108, 34, 58, 34, 98, 97,
            114, 34, 125,
        ];
        let expected = Redirect {
            code: String::from("foo"),
            created_at: 0,
            url: String::from("bar"),
        };
        let actual = serializer.decode(&data).unwrap();
        assert_eq!(expected, actual);
    }
}
