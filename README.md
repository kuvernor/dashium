# Dashium

A Geometry Dash Private Server written in Rust.

> Note: The project is in its early stages.

## How to try

### Requirements

- Rust
- PostgreSQL database
- `sqlx-cli`

### Setup

1. Create a PostgreSQL database and put the credentials in the `.env` file.
2. Run `sqlx migrate run` in this directory.

Then, run `cargo run --release` to start the server at `127.0.0.1:2207`.