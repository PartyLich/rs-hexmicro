Hexagonal microservice example

A simple URL shortener ported from Go. It's a nice supplement to books and articles like _Clean Architecture_ and in this case, the Rust Book (which I hadn't completed when wrote this).

I had done some research about the server frameworks available, then didn't have my notes available when I actually had time to implement that portion. I probably wouldn't choose Warp again. The problems it seeks to solve aren't necessarily problems I've had, and the route composition style doesn't address any personal need.

### Building the docs

I haven't hosted them anywhere, so

```
cargo docs --no-deps --open
```

### building the code

```
cargo build
```

### trying it locally

```
# start respository
  redis-server
# start webserver with env variables for config
  URL_DB=redis REDIS_URL=redis://localhost:6379 cargo run
  URL_DB=mongo MONGO_DB=shortener MONGO_TIMEOUT=45 MONGO_URL="<mongodb connection string>" cargo run
```

then use GET and POST however you please (curl, postman, etc)

### Contribute

Contributions are welcome. File an issue or PR.
