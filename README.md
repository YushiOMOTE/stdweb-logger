# stdweb-logger

Logger for stdweb

```rust
use log::*;

fn main() {
    stdweb::initialize();
    
    // Set logger
    stdweb_logger::init();

    // Log
    info!("{} -> {}", "Chin", "Tama");

    stdweb::event_loop();
}
```
