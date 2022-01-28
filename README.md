# README

This can either be run using either Rust (1) or via Docker (2)

## Running via Rust

Install Rust using [rustup](https://rustup.rs/)

Change into the root directory of this project and run using cargo:

```bash
cargo run
```

Then navigate to the url sepecified in the prompt.

Example URLs:
- http://127.0.0.1:3030/pokemon/mewtwo
- http://127.0.0.1:3030/pokemon/translated/mewtwo
- http://127.0.0.1:3030/pokemon/pikachu
- http://127.0.0.1:3030/pokemon/translated/pikachu

## Running via Docker

Run `docker build --tag technical-pokemon .` to build the Docker image.

Run `docker run -p 3030:3030 technical-pokemon` to run the Docker image.

## Improvements for Production

1. Better debug logging with structure
2. Tracing in production would log to another server such as New Relic.
3. Use `https`