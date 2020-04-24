use std::sync::{Arc, Mutex};
use warp::Filter;

use hex_microservice::{
    api, api::RedirectHandler, repository, short_url, short_url::RedirectService,
};

/// Get http port from environment variable. Default to port 8000
fn http_port() -> u16 {
    if let Ok(p) = std::env::var("PORT") {
        return p.parse::<u16>().unwrap_or(8000);
    }
    8000
}

/// Create repository instance according to environment variable. Default to MongoDB
fn choose_repo() -> Box<dyn short_url::RedirectRepository + Send> {
    let db = std::env::var("URL_DB").expect("URL_DB var must be provided");
    println!("db: {}", std::env::var("URL_DB").unwrap());

    match db.as_str() {
        "redis" => {
            let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL var must be provided");
            repository::RedisRepository::new(&redis_url).unwrap()
        }
        // "mongo" => {
        _ => {
            let mongo_url = std::env::var("MONGO_URL").expect("MONGO_URL var must be provided");
            let mongo_db = std::env::var("MONGO_DB").expect("MONGO_DB var must be provided");

            repository::MongoRepository::new(&mongo_url, &mongo_db).unwrap()
        }
    }
}

fn test_service(service: &dyn RedirectService) {
    const TEST_CODE: &str = "woah";
    let x = short_url::Redirect {
        code: String::from(TEST_CODE),
        created_at: 0,
        url: String::from("https://www.google.com"),
    };
    println!("{:?}", x);

    if let Err(e) = service.store(&x) {
        panic!(e)
    };

    println!("Stored test redirect {:?}", x);

    match service.find(TEST_CODE) {
        Ok(r) => {
            println!("found redirect: {:?}", r);
        }
        Err(e) => {
            println!("Unable to find redirect with code {}:\n\t {}", TEST_CODE, e);
        }
    }
}

#[tokio::main]
async fn main() {
    let repo = choose_repo();
    let service = short_url::Service::new(repo);

    test_service(&service);

    // Set up and start server
    let port = http_port();
    const SERVER_IP: [u8; 4] = [127, 0, 0, 1];
    let handler = Arc::new(Mutex::new(api::Handler::new(Box::new(service))));

    // Route definitions
    // GET /:String
    let get_handler = handler.clone();
    let get_code = warp::get()
        .and(warp::path::param())
        .and(warp::path::end())
        .map(move |code| {
            println!("get handler\n\tcode: {}", code);
            get_handler.lock().unwrap().get(code)
        });
    // POST /
    let post_handler = handler.clone();
    let post_code = warp::post()
        .and(warp::path::end())
        .and(warp::header("Content-Type"))
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::bytes())
        .map(move |content_type, req_body| {
            println!(
                "post handler\n\tcontent_type: {} req_body: {:?}",
                content_type, req_body
            );
            post_handler.lock().unwrap().post(content_type, req_body)
        });

    let api = get_code.or(post_code);

    // Start http server
    println!("Starting server on port {}", port);
    warp::serve(api).run((SERVER_IP, port)).await;
}
