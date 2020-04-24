//! `api` defines an http adapter for a url shortening service.

use bytes::Bytes;
use warp::http::{Response, StatusCode, Uri};

use crate::{
    serializer,
    short_url::{error::RedirectErr, RedirectSerializer, RedirectService},
};

/// Provides methods for driving a url shortening service
pub trait RedirectHandler {
    /// Handle GET requests for shortened Url redirects
    fn get(&self, code: String) -> Box<dyn warp::Reply>;

    /// Handle POST requests to create shorteneed Urls
    fn post(&self, content_type: String, req_body: Bytes) -> Box<dyn warp::Reply>;
}

/// A concrete implementation of the `RedirectHandler` interface
pub struct Handler {
    redirect_service: Box<(dyn RedirectService + Send)>,
}

impl Handler {
    /// Creates an instance of the `Handler` type
    pub fn new(redirect_service: Box<(dyn RedirectService + Send)>) -> Handler {
        Handler { redirect_service }
    }

    /// Creates appropriate http response header
    fn setup_response(
        content_type: String,
        body: Bytes,
        status_code: StatusCode,
    ) -> Response<Bytes> {
        Response::builder()
            .header("Content-Type", content_type)
            .status(status_code)
            .body(body)
            .expect("Failed to build http response")
    }

    /// Get a `RedirectSerializer` for the specified content type
    fn serializer(content_type: &str) -> Box<dyn RedirectSerializer> {
        match content_type {
            "application/x-msgpack" => Box::new(serializer::MsgpackSerializer {}),
            _ => Box::new(serializer::JsonSerializer {}),
        }
    }

    /// Create error response
    fn reply_with(e: RedirectErr) -> Box<dyn warp::Reply> {
        let status_code = match e {
            RedirectErr::NotFound => StatusCode::NOT_FOUND,
            RedirectErr::Invalid => StatusCode::BAD_REQUEST,
            RedirectErr::ServerErr => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Box::new(
            Response::builder()
                .status(status_code)
                .body(format!("{}", e))
                .expect("Failed to build http response"),
        )
    }
}

impl RedirectHandler for Handler {
    /// Redirect to stored url
    fn get(&self, code: String) -> Box<dyn warp::Reply> {
        match self.redirect_service.find(&code) {
            Ok(redirect) => {
                log::info!("redirect {}", redirect.url);
                Box::new(warp::redirect(
                    redirect
                        .url
                        .parse::<Uri>()
                        .expect(&format!("Parsing URI: {}", redirect.url)),
                ))
            }
            Err(e) => {
                log::warn!("Failed to redirect for code {}:\n\t{}", code, e);
                if e == RedirectErr::NotFound {
                    return Handler::reply_with(e);
                }
                server_err_reply()
            }
        }
    }

    /// Create short code for url redirect
    fn post(&self, content_type: String, req_body: Bytes) -> Box<dyn warp::Reply> {
        let redirect = match Handler::serializer(&content_type).decode(&req_body.slice(..).to_vec())
        {
            Ok(r) => r,
            Err(e) => {
                log::error!("redirect decode err: {}", e);
                return server_err_reply();
            }
        };

        let redirect = match self.redirect_service.store(&redirect) {
            Ok(r) => r,
            Err(e) => {
                log::error!("{}", e);
                match e {
                    RedirectErr::Invalid => return Handler::reply_with(e),
                    _ => return server_err_reply(),
                }
            }
        };

        let res_body = match Handler::serializer(&content_type).encode(&redirect) {
            Ok(b) => b,
            Err(e) => {
                log::error!("redirect encode error: {}", e);
                return server_err_reply();
            }
        };

        Box::new(Handler::setup_response(
            content_type,
            Bytes::from(res_body),
            StatusCode::CREATED,
        ))
    }
}

fn server_err_reply() -> Box<dyn warp::Reply> {
    Box::new(
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("{}", RedirectErr::ServerErr))
            .unwrap(),
    )
}
