# README

This can either be run using either Rust (1) or via Docker (2)

## Running via Rust

Install Rust using [rustup](https://rustup.rs/)

Change into the root directory of this project and run using cargo:

```bash
cargo run
```

Then navigate to the url sepecified in the prompt.

## Running via Docker

TODO

## Improvements for Production

1. Better debug logging with structure
2. ~~Tracing~~
  1. Enabled now, however in production this would log to another server such as New Relic.
3. Use `https`
4. Convert to using traits for PokeApi and FunTranslation