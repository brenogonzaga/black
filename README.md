Collecting workspace information

# Black

A simple logging library implemented in Rust.

## About

The **Black** project is a simple logging library that allows you to record log messages at different levels (DEBUG, INFO, WARN, EVENT, and ERROR) to a file. Logging is performed asynchronously and in a thread-safe manner using `Arc<Mutex<File>>`.

## Features

- Supports multiple log levels:
  - DEBUG
  - INFO
  - WARN
  - EVENT
  - ERROR
- Provides a simple and intuitive interface for event logging.

## Project Structure

- [Cargo.toml](Cargo.toml) – Project configuration and dependencies.
- [src/lib.rs](src/lib.rs) – Implementation of the **Black** library.
- [examples/main.rs](examples/main.rs) – Example usage of the library.

### Usage Example

Below is an example application that uses Black to record logs:

```rust
use anyhow::Result;
use black::BlackBox;

fn main() -> Result<()> {
    let logger = BlackBox::new("blackbox.log")?;

    logger.log_event("Application started")?;
    logger.log_error("Error: could not connect to the database")?;
    logger.log_event("Data processing complete")?;

    Ok(())
}
```

## Building and Running

To build the project, run:

```sh
cargo build
```

To run the example, execute:

```sh
cargo run --example main
```

## Tests

To run the tests (if available), execute:

```sh
cargo test
```

## License

Distributed under the MIT License.

## Contribution

Contributions are welcome! Please open an issue or submit a pull request.
