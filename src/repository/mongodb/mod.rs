use bson::{bson, doc};
use mongodb::{options::ClientOptions, Client};
use std::error::Error;

use crate::short_url::{error::RedirectErr, Redirect, RedirectRepository};

impl From<mongodb::error::Error> for RedirectErr {
    fn from(e: mongodb::error::Error) -> RedirectErr {
        log::error!("mongodb error {}", e);
        RedirectErr::ServerErr
    }
}

impl From<bson::DecoderError> for RedirectErr {
    fn from(e: bson::DecoderError) -> RedirectErr {
        log::error!("bson decode error {}", e);
        RedirectErr::ServerErr
    }
}

impl From<bson::EncoderError> for RedirectErr {
    fn from(e: bson::EncoderError) -> RedirectErr {
        log::error!("bson encode error {}", e);
        RedirectErr::ServerErr
    }
}

/// A Mongo backed `RedirectRepository` object
pub struct MongoRepository {
    client: mongodb::Client,
    database: String,
}

impl MongoRepository {
    fn new_mongo_client(mongo_url: &str) -> Result<Client, Box<dyn Error>> {
        let client_options = ClientOptions::parse(mongo_url)?;
        let client = Client::with_options(client_options)?;

        Ok(client)
    }

    /// Creates an instance of `RedirectRepository` backed by MongoDB
    pub fn new(mongo_url: &str, mongo_db: &str) -> Result<Box<MongoRepository>, Box<dyn Error>> {
        // generate new db client
        let client = MongoRepository::new_mongo_client(mongo_url)?;
        Ok(Box::new(MongoRepository {
            client,
            database: mongo_db.to_string(),
        }))
    }
}

impl RedirectRepository for MongoRepository {
    /// Look up URL based on its short code
    fn find(&self, code: &str) -> Result<Redirect, RedirectErr> {
        let collection = self.client.database(&self.database).collection("redirects");

        // search db for code
        let filter = doc! { "code": code };
        let result = collection.find_one(filter, None)?;
        match result {
            Some(document) => {
                let redirect = bson::from_bson(bson::Bson::Document(document))?;
                Ok(redirect)
            }
            None => Err(RedirectErr::NotFound),
        }
    }

    /// Save `Redirect` object to the repository
    fn store(&self, redirect: &Redirect) -> Result<(), RedirectErr> {
        let collection = self.client.database(&self.database).collection("redirects");

        // save redirect to the db
        let redirect_bson = bson::to_bson(redirect)?;

        if let bson::Bson::Document(doc) = redirect_bson {
            collection.insert_one(doc, None)?;
            Ok(())
        } else {
            log::error!("Error converting BSON object to MongoDB document");
            return Err(RedirectErr::ServerErr);
        }
    }
}
