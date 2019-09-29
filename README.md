# stdweb-logger

Logger for stdweb

[![Latest version](https://img.shields.io/crates/v/stdweb-logger.svg)](https://crates.io/crates/stdweb-logger)
[![Documentation](https://docs.rs/stdweb-logger/badge.svg)](https://docs.rs/stdweb-logger)
![License](https://img.shields.io/crates/l/stdweb-logger.svg)

```rust
use log::*;

fn main() {
    stdweb::initialize();
    
    // Set logger
    stdweb_logger::init();

    // Log
    info!("{}!", "Hello");

    stdweb::event_loop();
}
```
