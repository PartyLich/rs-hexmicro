//!  `msgpack` provides a concrete implementation of the RedirectSerializer interface

use rmp_serde as rmps;
use std::error::Error;

use crate::short_url::{Redirect, RedirectSerializer};

/// A concrete implementation of the `RedirectSerializer` interface that uses MsgPack as its protocol
pub struct MsgpackSerializer();

impl RedirectSerializer for MsgpackSerializer {
    /// Converts a `Redirect` to a MessagePack byte sequence
    fn encode(&self, input: &Redirect) -> Result<Vec<u8>, Box<dyn Error>> {
        rmps::to_vec_named(input).map_err(|e| e.into())
    }

    /// Converts a MessagePack byte sequence to a `Redirect`
    fn decode(&self, input: &Vec<u8>) -> Result<Redirect, Box<dyn Error>> {
        rmps::from_read_ref(input).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        let expected = Redirect {
            code: String::from("foo"),
            created_at: 0,
            url: String::from("bar"),
        };
        let serializer = MsgpackSerializer {};
        let encoded = serializer.encode(&expected).unwrap();
        let actual = serializer.decode(&encoded).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn encodes() {
        let serializer = MsgpackSerializer {};
        let expected = "[131, 170, 99, 114, 101, 97, 116, 101, 100, 95, 97, 116, 0, 164, 99, 111, 100, 101, 163, 102, 111, 111, 163, 117, 114, 108, 163, 98, 97, 114]";
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
        let serializer = MsgpackSerializer {};
        let data: Vec<u8> = vec![
            131, 170, 99, 114, 101, 97, 116, 101, 100, 95, 97, 116, 0, 164, 99, 111, 100, 101, 163,
            102, 111, 111, 163, 117, 114, 108, 163, 98, 97, 114,
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
